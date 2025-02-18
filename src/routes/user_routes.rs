use crate::handlers::post_handler::{add_post, upload_image};
use crate::handlers::user_handlers::{delete_user, get_users, update_user};
use axum::http::Method;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    Router::new()
        .route("/api/user/{uuid}/update", put(update_user))
        .route("/api/user/{uuid}/delete", delete(delete_user))
        .route("/api/users", get(get_users))
        .route("/api/user/posts", post(add_post))
        .route("/api/user/posts/{uuid}/image", post(upload_image))
        .layer(cors)
}
