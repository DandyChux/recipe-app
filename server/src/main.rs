mod handlers;
mod model;
mod routes;
mod config;
mod utils;
mod middleware;

use std::sync::Arc;
use tokio::sync::RwLock;
use config::Config;

use axum::{
    http::{
        header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE },
        HeaderValue, 
        Method,
        Request
    }, body::Body, middleware::from_fn, Extension, Router
};
use dotenv::{dotenv, from_filename};
use tower_http::{
    compression::CompressionLayer, cors::{Any, CorsLayer}, trace::TraceLayer
};
use crate::middleware::auth;
use tracing::{info, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        // Load from `.env.local` in development
        from_filename(".env.local").ok().expect("Failed to load .env.local file");
    } else {
        // Load from `.env` in production
        dotenv().ok().expect("Failed to load .env file");
    }

    let config = Config::init();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_static_web_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
        .allow_origin(std::env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()).parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE]);

    let app_state = Arc::new(RwLock::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }));

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .unwrap_or_else(|err| panic!("Failed to run migrations: {}", err));
    let mut migrations = match sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        {
            Ok(migrations) => {
                println!("✅Migrations ran successfully");
                migrations
            },
            Err(err) => {
                println!("❌Failed to run migrations: {}", err);
                return;
            }
        };

    let app = Router::new()
        .merge(routes::user_routes::user_routes())
        .merge(routes::auth_routes::auth_routes())
        // middleware
        .layer(from_fn(auth))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().on_request(|req: &Request<Body>, _span: &Span| {
                let addr = req
                    .headers()
                    .get("X-Real-IP")
                    .and_then(|ip| ip.to_str().ok());
                info!(
                    "[{}] {} {}",
                    addr.unwrap_or("unknown"),
                    req.method(),
                    req.uri()
                );
            }),
        )
        .layer(CompressionLayer::new())
        .layer(Extension(app_state));

    println!("🚀 Server started succesfully");
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}