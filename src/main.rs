use axum::{
    routing::{get, post},
    extract::{DefaultBodyLimit, State},
    http::{header, Method},
    Json, Router,
};

use passcore::{grade_password, review_password, score};

use serde::{Deserialize, Serialize};

use sqlx::SqlitePool;

use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};

use tracing::{error, info, Level};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Input {
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    score: u16,
    feedback: String,
    grade: String,
}

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

async fn health() -> &'static str {
    "I am healthy!"
}

async fn score_password(State(state): State<AppState>, Json(input): Json<Input>) -> Json<Output> {
    let password = input.password;
    let psw_score = score(&password);
    let review = review_password(&password).to_string();
    let grade = grade_password(&password).to_string();
    let output = Output {
        score: psw_score,
        feedback: review,
        grade,
    };

    if let Err(err) = sqlx::query(
        r#"
        UPDATE metrics
        SET total_scored = total_scored + 1,
            total_score_sum = total_score_sum + ?
        WHERE id = 1
        "#
    )
        .bind(psw_score as i64)
        .execute(&state.db)
        .await
    {
        error!(error = %err, "Failed to update metrics");
    } else {
        info!(score = psw_score, grade = %output.grade, "Password scored successfully");
    }

    Json(output)
}


#[derive(Serialize)]
struct StatsOutput {
    total_scored: i64,
    average_score: f64,
}

async fn get_stats(State(state): State<AppState>) -> Json<StatsOutput> {
    let row = sqlx::query_as::<_, (i64, i64)>(
        r#"
        SELECT total_scored, total_score_sum
        FROM metrics
        WHERE id = 1
        "#
    )
        .fetch_one(&state.db)
        .await
        .expect("Failed to fetch stats");

    let (total_scored, total_score_sum) = row;

    let average_score = if total_scored > 0 {
        total_score_sum as f64 / total_scored as f64
    } else {
        0.0
    };

    Json(StatsOutput {
        total_scored,
        average_score,
    })
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "passcore_web=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = std::env::var("DATABASE_PATH").unwrap_or("db.sqlite".to_string());

    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("Failed to create DB directory");
        }
    }

    if !std::path::Path::new(&db_path).exists() {
        std::fs::File::create(&db_path).expect("Failed to create db file");
    }

    info!(db_path = %db_path, "Using database path");

    let db_url = format!("sqlite://{}", db_path);
    let db = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metrics (
            id INTEGER PRIMARY KEY,
            total_scored INTEGER NOT NULL,
            total_score_sum INTEGER NOT NULL
        );
        "#
    )
        .execute(&db)
        .await
        .expect("Failed to create metrics table);");

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO metrics (id, total_scored, total_score_sum)
        VALUES (1, 0, 0);
        "#
    )
        .execute(&db)
        .await
        .expect("Failed to initialize metrics row");

    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_origin([
            "https://passcore-web-production.up.railway.app".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    use tower_governor::{
        governor::GovernorConfigBuilder,
        GovernorLayer,
    };

    // For local dev
    #[cfg(debug_assertions)]
    use tower_governor::key_extractor::PeerIpKeyExtractor;

    #[cfg(not(debug_assertions))]
    use tower_governor::key_extractor::SmartIpKeyExtractor;

    // for local dev
    #[cfg(debug_assertions)]
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(15)
        .key_extractor(PeerIpKeyExtractor)
        .use_headers()
        .finish()
        .unwrap();

    #[cfg(not(debug_assertions))]
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(4)
        .burst_size(25)
        .key_extractor(SmartIpKeyExtractor)
        .use_headers()
        .finish()
        .unwrap();

    let score_routes = Router::new()
        .route("/score", post(score_password))
        .layer(GovernorLayer::new(governor_conf))
        .layer(DefaultBodyLimit::max(1024));

    let app = Router::new()
        .route("/health", get(health))
        .route("/stats", get(get_stats))
        .merge(score_routes)
        .fallback_service(ServeDir::new("static"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                )
        )
        .layer(cors)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .unwrap();
}