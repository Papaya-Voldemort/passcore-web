use axum::{
    routing::get,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use passcore::{score, review_password, grade_password};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

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


async fn health() -> &'static str {
    "I am healthy!"
}

async fn score_password(Json(input): Json<Input>) -> Json<Output> {
    let password = input.password;
    let psw_score = score(&password);
    let review = review_password(&password).to_string();
    let grade = grade_password(&password).to_string();
    let output = Output {
        score: psw_score,
        feedback: review,
        grade,
    };
    Json(output)
}


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/score", post(score_password))
        .fallback_service(ServeDir::new("static"))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}