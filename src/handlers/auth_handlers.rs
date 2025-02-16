use crate::models::user::{LoginResponse, User, UserCreateModel, UserLoginModel};
use crate::utils::errors::APIError;
use crate::utils::jwt::encode_jwt;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_query::Condition;
use uuid::Uuid;

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<UserCreateModel>,
) -> Result<Json<User>, APIError> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    match user {
        None => {
            let new_user = entity::user::ActiveModel {
                id: Default::default(),
                name: Set(data.name),
                email: Set(data.email),
                password: Set(data.password),
                uuid: Set(Uuid::new_v4()),
                created_at: Set(Utc::now().naive_utc()),
            };

            new_user.clone().insert(&db).await.map_err(|err| APIError {
                message: err.to_string(),
                status_code: StatusCode::CONFLICT,
                error_code: Some(40),
            })?;

            // TODO: implement a serializer on `User`
            let response = User {
                name: new_user.name.clone().unwrap(),
                email: new_user.email.clone().unwrap(),
                password: new_user.password.clone().unwrap(),
                uuid: new_user.uuid.clone().unwrap(),
                created_at: new_user.created_at.clone().unwrap(),
            };

            Ok(Json(response))
        }
        Some(existing_user) => Err(APIError {
            message: format!("User {:?} exists", existing_user.name),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        }),
    }
}

pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<UserLoginModel>,
) -> Result<Json<LoginResponse>, APIError> {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(data.email))
                .add(entity::user::Column::Password.eq(data.password)),
        )
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "User not Found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    let token = encode_jwt(user.email).map_err(|_| APIError {
        message: "login failed".to_string(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    Ok(Json(LoginResponse { token }))
}
