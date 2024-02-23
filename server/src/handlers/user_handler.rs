use axum::{
    http::{ header, Response, StatusCode },
    response::IntoResponse,
    routing::get,
    Json, 
    Router,
    Extension
};
use crate::AppState;
use std::sync::Arc;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rusty Melody is healthy!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}

pub async fn get_user_preferences_handler(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    // Add logic to get user preferences
}

pub async fn update_user_preferences_handler(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    // Add logic to update user preferences
}