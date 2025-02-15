use crate::models::user::{User, UserUpdateModel};
use crate::utils::errors::APIError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(data): Json<UserUpdateModel>,
) -> Result<Json<User>, APIError> {
    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    user.name = Set(data.name);
    user.clone().update(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    // since we cannot convert `ActiveModel` to `Model`, we do that manually
    let user = user.clone();
    let response = User {
        name: user.name.unwrap(),
        email: user.email.unwrap(),
        password: user.password.unwrap(),
        uuid: user.uuid.unwrap(),
        created_at: user.created_at.unwrap(),
    };

    Ok(Json(response))
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
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
    // db.close().await.unwrap();

    (StatusCode::OK, Json("deleted"))
}

pub async fn get_users(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
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

    // db.close().await.unwrap();
    (StatusCode::OK, Json(users))
}
