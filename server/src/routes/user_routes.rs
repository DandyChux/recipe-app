use axum::routing::{delete, put, get, post};
use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers::user_handler::{
    get_user_preferences_handler, 
    update_user_preferences_handler,
    health_check_handler
};

pub fn user_routes() -> Router {
    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/healthchecker", get(health_check_handler))
    .route("/user/preferences", get(get_user_preferences_handler))
    .route("/user/preferences", put(update_user_preferences_handler))
    .layer(cors);

    router
}