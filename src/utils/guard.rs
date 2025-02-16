use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use headers::authorization::Bearer;
use headers::{Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{errors::APIError, jwt::decode_jwt};

pub async fn guard<T>(mut req: Request<Body>, next: Next) -> Result<Response, APIError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError {
            message: "No Auth token found".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|_| APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?
        .claims;

    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(APIError {
            message: "Could not connect to database".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
