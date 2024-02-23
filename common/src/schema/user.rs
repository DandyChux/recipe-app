use serde::{ Deserialize, Serialize };
use chrono::prelude::*;

#[derive(Debug, Deserialize)]
pub struct SignupUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub preferred_platform: Option<String>,
    pub photo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub preferred_platform: String,
    pub photo: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}