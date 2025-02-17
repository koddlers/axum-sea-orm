use crate::models::user::UserRelationModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Post {
    pub uuid: Uuid,
    pub text: String,
    pub image: String,
    pub title: String,
    pub user: Option<UserRelationModel>,
}

#[derive(Serialize, Deserialize)]
pub struct PostCreateModel {
    pub text: String,
    pub image: String,
    pub title: String,
}

impl From<(entity::post::Model, Option<entity::user::Model>)> for Post {
    fn from(value: (entity::post::Model, Option<entity::user::Model>)) -> Self {
        let user_model = value.1.unwrap();
        Self {
            uuid: value.0.uuid,
            text: value.0.text,
            image: value.0.image,
            title: value.0.title,
            user: Some(UserRelationModel {
                name: user_model.name,
                email: user_model.email,
                uuid: user_model.uuid,
            }),
        }
    }
}
