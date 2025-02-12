use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginModel {
    pub email: String,
    pub password: String,
}
