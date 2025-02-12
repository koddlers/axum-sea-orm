use axum;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    server().await;
}

async fn server() {
    let router = Router::new().route("/api/test", get(test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("Test Api");
    (StatusCode::ACCEPTED, "Hello There")
}
