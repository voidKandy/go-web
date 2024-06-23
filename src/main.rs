mod auth;
mod database;
mod error;
mod routing;
mod state;
mod telemetry;
use auth::handlers::hash_password;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use core::panic;
use database::handlers::register_new_user;
use error::IntoDataApiReturn;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use state::AppState;
use state::*;
use std::sync::Arc;
use telemetry::{get_subscriber, init_subscriber};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "main".to_string();

    if std::env::var("MAIN_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});
#[tokio::main]
async fn main() {
    Lazy::force(&TRACING);
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").expect("Failed to get port env variable");

    let config = Config::init();
    tracing::info!(
        "attempting to connect to database: {:?}",
        &config.database_url
    );
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to migrate database");

    let admin_pass = std::env::var("ADMIN_PASSWORD").expect("Failed to get admin password");
    let admin_name = std::env::var("ADMIN_NAME").expect("Failed to get admin name");
    let admin_email = std::env::var("ADMIN_EMAIL").expect("Failed to get admin email");
    info!("Got admin info");

    match register_new_user(
        &pool,
        &admin_name,
        &admin_email,
        true,
        &hash_password(&admin_pass).expect("Failed to hash password"),
        true,
    )
    .await
    {
        Ok(_) => info!("Created admin"),
        Err(err) => {
            if err.status_code() == StatusCode::CONFLICT {
                warn!("Admin already created")
            } else {
                panic!("Unexpected error when creating admin: {:?}", err)
            }
        }
    }

    let allowed_origin = std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| {
        warn!("No allowed origin env var, falling back to localhost");
        "http://localhost:3000".to_string()
    });

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = routing::create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }))
    .layer(cors);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
