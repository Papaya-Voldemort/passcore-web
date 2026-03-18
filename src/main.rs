use axum::{
    routing::get,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use passcore::{score, review_password, grade_password};

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

async fn root() -> &'static str {
    "Please call the /score endpoint!"
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
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/score", post(score_password));

    // run our app with hyper, listening globally on port 3000
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}