use crate::handlers::post_handler::get_post;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn home_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    Router::new()
        .route("/api/post/{uuid}", get(get_post))
        .layer(cors)
}
