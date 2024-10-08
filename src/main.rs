mod error;
mod hilighting;
mod routing;
mod telemetry;
use std::sync::LazyLock;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use telemetry::{get_subscriber, init_subscriber};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

static TRACING: LazyLock<()> = LazyLock::new(|| {
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
    LazyLock::force(&TRACING);
    info!("tracing initialized");
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").expect("Failed to get port env variable");

    // tracing::info!(
    //     "attempting to connect to database: {:?}",
    // &config.database_url
    // );

    let allowed_origin = std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| {
        warn!("No allowed origin env var, falling back to localhost");
        "http://localhost:3000".to_string()
    });

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // let app = routing::create_router(Arc::new(AppState {
    //     env: config.clone(),
    // }))
    // .layer(cors);

    let app = routing::create_router();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
