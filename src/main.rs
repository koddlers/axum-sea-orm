mod handlers;
mod models;
mod routes;
mod utils;

use axum;
use axum::{Extension, Router};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let db_conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(db_conn_str)
        .await
        .expect("FATAL: Failed to connect to database");

    let router = Router::new()
        .merge(routes::auth_router::auth_routes())
        .merge(routes::user_routes::user_routes())
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
