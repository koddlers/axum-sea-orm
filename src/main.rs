mod handlers;
mod models;
mod routes;
mod utils;

use axum;
use axum::body::Body;
use axum::routing::get_service;
use axum::{middleware, Extension, Router};
use sea_orm::Database;
use tower_http::services::ServeDir;

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
        .merge(routes::user_routes::user_routes())
        .route_layer(middleware::from_fn(utils::guard::guard::<Body>))
        .merge(routes::auth_router::auth_routes())
        .merge(routes::home_routes::home_routes())
        .layer(Extension(db))
        .fallback_service(get_service(ServeDir::new("public")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
