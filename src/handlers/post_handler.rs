use crate::models::post::{Post, PostCreateModel};
use crate::models::user::UserRelationModel;
use crate::utils::errors::APIError;
use axum::extract::{Multipart, Path};
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect,
};
use sea_query::Condition;
use sea_query::JoinType;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use std::io::BufWriter;

use image::codecs::png::PngEncoder;
use image::{ImageEncoder, ImageReader};

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, Resizer};

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

pub async fn upload_image(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Path(uuid): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<(), APIError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // TODO: handle the possible error for `unwrap()`
        let field_name = field.name().unwrap().to_string();
        if field_name == "image" {
            let post = entity::post::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::post::Column::Uuid.eq(uuid))
                        .add(entity::post::Column::UserId.eq(identity.id)),
                )
                .one(&db)
                .await
                .unwrap()
                .unwrap();

            let image_name = Utc::now().timestamp();
            // TODO: handle the possible error for `unwrap()`
            let data = field.bytes().await.unwrap();

            // Read source image from file
            let src_image = ImageReader::new(std::io::Cursor::new(data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();

            // Create container for data of destination image
            let dst_width = 480;
            let dst_height = 360;
            let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type().unwrap());

            // Create Resizer instance and resize source image
            // into buffer of destination image
            let mut resizer = Resizer::new();
            resizer.resize(&src_image, &mut dst_image, None).unwrap();

            // Write destination image as PNG-file
            let mut result_buf = BufWriter::new(Vec::new());
            PngEncoder::new(&mut result_buf)
                .write_image(
                    dst_image.buffer(),
                    dst_width,
                    dst_height,
                    src_image.color().into(),
                )
                .unwrap();

            let image_bytes = result_buf.into_inner().unwrap();
            // TODO: add image path to `constants`
            let mut file = File::create(format!("./public/uploads/{}.png", image_name))
                .await
                .unwrap();
            // TODO: handle the possible error for `unwrap()`
            file.write(&image_bytes).await.unwrap();

            let mut post_model = post.into_active_model();
            post_model.image = Set(format!("./public/uploads/{}.png", image_name));
            post_model.update(&db).await.unwrap();
        } else {
            // TODO: handle the possible error for `unwrap()`
            let data = field.text().await.unwrap();
            println!("field: {}, \tvalue: {}", field_name, data);
        }
    }

    Ok(())
}
