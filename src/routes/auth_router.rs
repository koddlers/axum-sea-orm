use crate::handlers::auth_handlers::{create_user, login_user};
use axum::http::Method;
use axum::routing::post;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/api/user/register", post(create_user))
        .route("/api/user/login", post(login_user))
        .layer(cors)
}
