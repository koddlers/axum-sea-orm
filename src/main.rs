mod models;

use crate::models::user::{User, UserCreateModel, UserLoginModel, UserUpdateModel};
use axum;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::Utc;
use entity::user;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, EntityTrait, QueryFilter};
use sea_query::Condition;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {
    let router = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/create", post(create_user))
        .route("/api/user/login", post(login_user))
        .route("/api/user/{uuid}/update", put(update_user))
        .route("/api/user/{uuid}/delete", delete(delete_user))
        .route("/api/users", get(get_users));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn test() -> impl IntoResponse {
    println!("Test Api");
    (StatusCode::ACCEPTED, "Hello There")
}

async fn create_user(Json(data): Json<UserCreateModel>) -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let user = user::ActiveModel {
        id: Default::default(),
        name: Set(data.name),
        email: Set(data.email),
        password: Set(data.password),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
    };

    let data = user.insert(&db).await.unwrap();
    db.close().await.unwrap();

    let user = User {
        name: data.name,
        email: data.email,
        password: data.password,
        uuid: data.uuid,
        created_at: data.created_at,
    };

    (StatusCode::ACCEPTED, Json(user))
}

async fn login_user(Json(data): Json<UserLoginModel>) -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(data.email))
                .add(entity::user::Column::Password.eq(data.password)),
        )
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let response = User {
        name: user.name,
        email: user.email,
        password: user.password,
        uuid: user.uuid,
        created_at: user.created_at,
    };

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

async fn update_user(
    Path(uuid): Path<Uuid>,
    Json(data): Json<UserUpdateModel>,
) -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    user.name = Set(data.name);
    user.clone().update(&db).await.unwrap();
    db.close().await.unwrap();

    // since we cannot convert `ActiveModel` to `Model`, we do that manually
    let user = user.clone();
    let response = User {
        name: user.name.unwrap(),
        email: user.email.unwrap(),
        password: user.password.unwrap(),
        uuid: user.uuid.unwrap(),
        created_at: user.created_at.unwrap(),
    };

    (StatusCode::ACCEPTED, Json(response))
}

async fn delete_user(Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    entity::user::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .unwrap();
    db.close().await.unwrap();

    (StatusCode::OK, Json("deleted"))
}

async fn get_users() -> impl IntoResponse {
    let db = Database::connect("postgres://postgres:bloodyroots@localhost/axum-fullstack")
        .await
        .unwrap();

    let users: Vec<User> = entity::user::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|user| User {
            name: user.name,
            email: user.email,
            password: user.password,
            uuid: user.uuid,
            created_at: user.created_at,
        })
        .collect();

    db.close().await.unwrap();
    (StatusCode::OK, Json(users))
}
