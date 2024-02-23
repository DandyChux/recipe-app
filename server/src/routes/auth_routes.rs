use axum::routing::{ post };
use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers::auth_handler::{
    login_user_handler, 
    register_user_handler,
};

pub fn auth_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::POST])
    .allow_origin(Any);

    let router = Router::new()
    .route("/auth/login", post(login_user_handler))
    .route("/auth/register", post(register_user_handler))
    .layer(cors);

    router
}