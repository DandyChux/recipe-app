use serde::{ Deserialize, Serialize };
use chrono::prelude::*;
use validator::Validate;

use super::platform::Platform;

#[derive(Debug, Deserialize, Validate, Clone, Default, Serialize)]
pub struct SignupUserSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Password confirmation is required"),
        length(min = 6, message = "Password confirmation must be at least 6 characters"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub password_confirm: String,
    pub username: String,
    pub preferred_platform: Option<String>,
    pub photo: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub struct FilteredUser {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub preferred_platform: Platform,
    pub photo: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub access_token: String,
    pub refresh_token: String,
}

