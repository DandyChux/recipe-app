use axum::http::HeaderValue;
use axum::routing::{delete, put, get, post};
use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, AllowCredentials};

use crate::handlers::user_handler::{
    get_user_preferences_handler, 
    update_user_preferences_handler,
    health_check_handler,
    get_user_handler
};

pub fn user_routes() -> Router {
    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_credentials(true);

    let router = Router::new()
    .route("/api/healthchecker", get(health_check_handler))
    .route("/api/user/preferences", get(get_user_preferences_handler))
    .route("/api/user/info", get(get_user_handler))
    .route("/api/user/preferences", put(update_user_preferences_handler))
    .layer(cors);

    router
}