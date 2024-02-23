use axum::{
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, 
    Router,
    Extension
};
use crate::AppState;
use std::sync::Arc;

pub async fn register_user_handler(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    // Add logic to register a user to database
}

pub async fn login_user_handler(state: Extension<Arc<AppState>>) -> impl IntoResponse {
    // Add logic to login a user
}