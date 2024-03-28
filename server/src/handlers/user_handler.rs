use axum::{
    extract::Path, http::{ header, Response, StatusCode }, response::IntoResponse, routing::get, Extension, Json, Router
};
use axum_extra::{headers::Cookie, TypedHeader};
use crate::{model::Users, utils::jwt::{decode_token, AccessClaims, AuthError}, AppState};
use tokio::sync::RwLock;
use std::sync::Arc;
use common::schema::{feedback::ErrorResponse, platform::Platform, user::{FilteredUser, UserData, UserResponse}};
use serde_json::json;

pub async fn health_check_handler(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    const MESSAGE: &str = "Rusty Melody is healthy!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}

pub async fn get_user_preferences_handler(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    // Add logic to get user preferences
}

pub async fn update_user_preferences_handler(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    // Add logic to update user preferences
}

pub async fn get_user_handler(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Extension(state): Extension<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookies.get("access_token")
        .ok_or((StatusCode::UNAUTHORIZED, Json(ErrorResponse {
            status: "fail".to_string(),
            message: "Access token not found".to_string(),
        })))?;

    // Decode the token to extract the user_id
    let user_id = decode_token::<AccessClaims>(&token)
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "fail".to_string(),
                message: match e {
                    AuthError::InvalidToken => "Invalid access token".to_string(),
                    _ => "Unknown error".to_string(),
                },
            };

            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?
        .sub;

    let user = sqlx::query_as!(
        Users,
        "SELECT * FROM users WHERE user_id = $1",
        uuid::Uuid::parse_str(&user_id).unwrap()
    )
    .fetch_optional(&state.try_read().unwrap().db)
    .await
    .map_err(|e| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: format!("Database error: {}", e)
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = ErrorResponse {
            status: "fail".to_string(),
            message: "User not found".to_string(),
        };

        (StatusCode::NOT_FOUND, Json(error_response))
    })?;

    // Convert the preferred_platform from Option<String> to Option<Platform>
    let preferred_platform = user.preferred_platform.map(|s| Platform::from(s));

    let filtered_user = FilteredUser {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        name: user.name,
        photo: user.photo.unwrap_or_else(|| "".to_string()),
        preferred_platform: preferred_platform.unwrap_or(Platform::Spotify),
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap()
    };

    let response = json!(UserResponse {
        status: "success".to_string(),
        message: "User found".to_string(),
        data: UserData {
            user: filtered_user
        }
    });

    Ok(Json(response))
}