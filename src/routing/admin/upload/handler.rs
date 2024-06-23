use super::db_integration::{
    change_attachments_links, save_all_attachments_to_filesystem, TryFromMultipart,
};
use crate::{
    database::{
        handlers::{delete_post_by_title, patch_blog_post, post_blog_post},
        model::{UpdateBlogPost, UploadBlogPost},
    },
    error::{DataApiReturn, IntoDataApiReturn},
    routing::error::RouterError,
    state::AppState,
};
use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    response::{Html, IntoResponse, Response},
};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;
use tracing::{debug, error, warn};

#[derive(Template)]
#[template(path = "admin/upload.html")]
pub struct UploadTemplate;

#[derive(Template)]
#[template(path = "admin/update.html")]
pub struct UpdateTemplate {
    pub title: String,
}

pub async fn get_upload_form() -> Html<String> {
    match UploadTemplate.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}

#[tracing::instrument(name = "upload blog post handler", skip_all)]
pub async fn post_upload_form(
    State(data): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<impl IntoResponse, DataApiReturn> {
    let mut post = UploadBlogPost::try_from_multipart(multipart)
        .await
        .map_err(|err| {
            warn!(
                "Encountered an error trying to parse upload form: {:?}",
                err
            );
            err.into_data_api_return()
        })?;
    debug!("Got post from multipart form: {:?}", post);
    post.content =
        change_attachments_links(&post.content).map_err(|err| err.into_data_api_return())?;
    save_all_attachments_to_filesystem(post.attachments.drain(..).collect()).map_err(|err| {
        error!("an error occurred when saving attachments: {:?}", err);
        err.into_data_api_return()
    })?;
    let post = post_blog_post(&data.db, post).await.map_err(|err| {
        warn!("Encountered an error trying to post: {:?}", err);
        err.into_data_api_return()
    })?;

    debug!("posted post: {:?}", post);
    let response =
        Response::new(json!({"status": "success", "message": "blog post uploaded!"}).to_string());
    Ok(response)
}

pub async fn get_update_form(Path(title): Path<String>) -> Html<String> {
    let tmpl = UpdateTemplate { title };
    match tmpl.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}

pub async fn patch_upload_form(
    State(data): State<Arc<AppState>>,
    Path(title): Path<String>,
    multipart: Multipart,
) -> Result<impl IntoResponse, DataApiReturn> {
    let mut post = UpdateBlogPost::try_from_multipart(multipart)
        .await
        .map_err(|err| {
            warn!(
                "Encountered an error trying to parse upload form: {:?}",
                err
            );
            err.into_data_api_return()
        })?;
    if let Some(c) = post.content {
        post.content =
            Some(change_attachments_links(&c).map_err(|err| err.into_data_api_return())?);
    }
    save_all_attachments_to_filesystem(post.attachments.drain(..).collect()).map_err(|err| {
        error!("an error occurred when saving attachments: {:?}", err);
        err.into_data_api_return()
    })?;

    patch_blog_post(&data.db, &title, post)
        .await
        .map_err(|err| {
            warn!("Encountered an error trying to post: {:?}", err);
            err.into_data_api_return()
        })?;
    let response =
        Response::new(json!({"status": "success", "message": "blog post updated!"}).to_string());
    Ok(response)
}

pub async fn delete_post(
    State(data): State<Arc<AppState>>,
    Path(title): Path<String>,
) -> Result<impl IntoResponse, DataApiReturn> {
    delete_post_by_title(&data.db, &title)
        .await
        .map_err(|err| {
            warn!("Encountered an error trying to delete post: {:?}", err);
            err.into_data_api_return()
        })?;
    let response =
        Response::new(json!({"status": "success", "message": "blog post deleted!"}).to_string());
    Ok(response)
}
