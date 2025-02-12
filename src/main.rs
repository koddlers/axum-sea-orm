use axum;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use chrono::Utc;
use entity::user;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let router = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/create", get(get_users));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("Test Api");
    (StatusCode::ACCEPTED, "Hello There")
}

async fn get_users() -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let user = user::ActiveModel {
        id: Default::default(),
        name: Set("Shaphil".to_string()),
        email: Set("mahmud@shaphil.me".to_string()),
        password: Set("123456".to_string()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
    };

    let data = user.insert(&db).await.unwrap();
    (StatusCode::ACCEPTED, data.name)
}
