use axum;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack").await.unwrap();
    let router = Router::new().route("/api/test", get(test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("Test Api");
    (StatusCode::ACCEPTED, "Hello There")
}
