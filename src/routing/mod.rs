mod admin;
mod error;
mod index;
mod music;
mod pages;
mod user;
use crate::{
    auth::{
        handlers::{get_me_handler, login_user_handler, logout_handler, register_user_handler},
        middleware::{admin_auth, auth, soft_auth},
    },
    routing::index::IndexTemplate,
    AppState,
};
use askama::Template;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use pages::{blog, contact, landing, portfolio};
use std::sync::Arc;
use tower_http::services::ServeDir;

pub type HandlerResult<T> = Result<T, StatusCode>;

#[tracing::instrument(name = "create app router", skip_all)]
pub fn create_router(state: Arc<AppState>) -> Router {
    let blog_routes = Router::new()
        .route("/", get(blog::index))
        .route("/latest", get(blog::latest))
        .layer(middleware::from_fn_with_state(state.clone(), soft_auth));

    let admin_routes = Router::new()
        .route(
            "/upload",
            get(admin::upload::get_upload_form).post(admin::upload::post_upload_form),
        )
        .route(
            "/update/:title",
            get(admin::upload::get_update_form).patch(admin::upload::patch_upload_form),
        )
        .route("/delete/:title", delete(admin::upload::delete_post))
        .route_layer(middleware::from_fn_with_state(state.clone(), admin_auth));

    let user_routes = Router::new()
        .route(
            "/",
            get(user::get_user_template)
                .route_layer(middleware::from_fn_with_state(state.clone(), soft_auth)),
        )
        .route("/login", get(user::login_user))
        .route("/register", get(user::register_user));

    let data_routes = Router::new()
        .route("/auth/register", post(register_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route(
            "/auth/logout",
            get(logout_handler).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/users/me",
            get(get_me_handler).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        );

    let private_dir_router = Router::new()
        .nest_service("/", ServeDir::new("private"))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth));

    Router::new()
        .route("/", get(index::index))
        .route("/landing", get(landing::index))
        .route(
            "/music/:current_album_name/:current_song_idx",
            get(music::music_handler)
                .route_layer(middleware::from_fn_with_state(state.clone(), soft_auth)),
        )
        .route("/portfolio/:item_name", get(portfolio::index))
        .route("/email", get(contact::send_email))
        .nest("/blog", blog_routes)
        .nest("/admin", admin_routes)
        .route("/contact", get(contact::index))
        .route_layer(middleware::from_fn_with_state(state.clone(), soft_auth))
        .nest("/user", user_routes)
        .layer(middleware::from_fn(htmx_request_check))
        .nest("/data", data_routes)
        .nest("/private", private_dir_router)
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
