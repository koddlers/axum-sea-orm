mod handlers;
mod models;
mod routes;

use axum;
use axum::Router;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let router = Router::new()
        .merge(routes::auth_router::auth_routes())
        .merge(routes::user_routes::user_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
