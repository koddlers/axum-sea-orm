use crate::models::post::{Post, PostCreateModel};
use crate::models::user::UserRelationModel;
use crate::utils::errors::APIError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
use sea_query::JoinType;
use serde_json::Value;
use uuid::Uuid;

pub async fn add_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Json(data): Json<PostCreateModel>,
) -> Result<Json<Post>, APIError> {
    let post = entity::post::ActiveModel {
        id: Default::default(),
        uuid: Set(Uuid::new_v4()),
        title: Set(data.title),
        text: Set(data.text),
        image: Set(data.image),
        user_id: Set(identity.id),
        created_at: Set(Utc::now().naive_local()),
    };

    post.clone().insert(&db).await.map_err(|_| APIError {
        message: "Failed to create Post".to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    let post = post.clone();
    let user = UserRelationModel {
        name: identity.name,
        email: identity.email,
        uuid: identity.uuid,
    };

    let response = Post {
        text: post.text.unwrap(),
        image: post.image.unwrap(),
        title: post.title.unwrap(),
        user: Option::from(user),
        ..Default::default()
    };

    Ok(Json(response))
}

pub async fn get_post(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Value>, APIError> {
    let post = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(uuid))
        // .find_also_related(entity::user::Entity)
        .column_as(entity::user::Column::Name, "author")
        .column_as(entity::user::Column::Uuid, "author uuid")
        .join(
            JoinType::LeftJoin,
            entity::post::Entity::belongs_to(entity::user::Entity)
                .from(entity::post::Column::UserId)
                .to(entity::user::Column::Id)
                .into(),
        )
        .into_json()
        .one(&db)
        .await
        .map_err(|_| APIError {
            message: "Failed to create Post".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Post not Found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    Ok(Json(post))
}
