use axum::http::HeaderValue;
use axum::routing::{ post, get };
use axum::{Router, http::Method};
use tower_http::cors::{AllowCredentials, Any, CorsLayer};

use crate::handlers::auth_handler::{
    login_user_handler, 
    register_user_handler,
    logout_handler,
    refresh_token_handler
};

pub fn auth_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_credentials(true)
    .allow_methods([Method::POST]);

    let router = Router::new()
    .route("/api/auth/login", post(login_user_handler))
    .route("/api/auth/register", post(register_user_handler))
    .route("/api/auth/logout", post(logout_handler))
    .route("/api/auth/refresh", post(refresh_token_handler))
    .layer(cors);

    router
}