mod error;
mod index;
mod music;
mod pages;
use std::sync::Arc;

use crate::routing::index::IndexTemplate;
use askama::Template;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use pages::{
    contact, landing, portfolio,
    videos::{self, VideoInfo},
};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

pub type HandlerResult<T> = Result<T, StatusCode>;

pub struct AppState {
    videos_cache: Option<Vec<VideoInfo>>,
}

type SharedState = Arc<RwLock<AppState>>;

pub fn init_shared_state() -> SharedState {
    Arc::new(RwLock::new(AppState { videos_cache: None }))
}

#[tracing::instrument(name = "create app router", skip_all)]
pub fn create_router(state: SharedState) -> Router<()> {
    Router::new()
        .route("/", get(index::index))
        .route("/landing", get(landing::index))
        .route(
            "/music/:current_album_name/:current_song_idx",
            get(music::music_handler),
        )
        .route("/portfolio/:item_name", get(portfolio::index))
        .route("/videos", get(videos::videos))
        .route("/email", get(contact::send_email))
        .route("/contact", get(contact::index))
        .layer(middleware::from_fn(htmx_request_check))
        .fallback(index::custom_404)
        .with_state(state)
        .nest_service("/public", ServeDir::new("public"))
}

async fn htmx_request_check(headers: HeaderMap, req: Request, next: Next) -> Response {
    let uri = req.uri();

    if headers.get("Hx-Request").is_none() {
        let template = IndexTemplate::from(uri);
        tracing::info!(
            "HxRequest header not present, middleware returning HTML template: {:?}",
            template
        );
        return Html(template.render().unwrap()).into_response();
    }

    tracing::info!("HxRequest header present, passing through middleware...");
    next.run(req).await.into()
}
