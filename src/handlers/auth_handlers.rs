use crate::models::user::{User, UserCreateModel, UserLoginModel};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sea_query::Condition;
use uuid::Uuid;

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<UserCreateModel>,
) -> impl IntoResponse {
    let user = entity::user::ActiveModel {
        id: Default::default(),
        name: Set(data.name),
        email: Set(data.email),
        password: Set(data.password),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
    };

    let data = user.insert(&db).await.unwrap();
    // db.close().await.unwrap();

    let user = User {
        name: data.name,
        email: data.email,
        password: data.password,
        uuid: data.uuid,
        created_at: data.created_at,
    };

    (StatusCode::ACCEPTED, Json(user))
}

pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<UserLoginModel>,
) -> impl IntoResponse {
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

    // db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}
