mod handlers;
mod model;
mod routes;
mod config;
mod utils;
mod middleware;

use std::sync::Arc;
use config::Config;

use axum::{
    http::{
        header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE },
        HeaderValue, 
        Method 
    }, middleware::from_fn, Extension, Router
};
use dotenv::{dotenv, from_filename};
use tower_http::cors::CorsLayer;
use crate::middleware::auth;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    // Conditional function to load env file based on environment
    if cfg!(debug_assertions) {
        // Load from `.env.local` in development
        from_filename(".env.local").ok().expect("Failed to load .env.local file");
    } else {
        // Load from `.env` in production
        dotenv().ok().expect("Failed to load .env file");
    }

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful");
            pool
        }
        Err(err) => {
            println!("❌Failed to connect to the database: {}", err);
            return;
        }
    };

    let cors = CorsLayer::new()
        // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        // .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        // .allow_credentials(true)
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    });

    let app = Router::new()
        .merge(routes::user_routes::user_routes())
        .merge(routes::auth_routes::auth_routes())
        .layer(from_fn(auth))
        .layer(Extension(app_state))
        .layer(cors);

    println!("🚀 Server started succesfully");
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
