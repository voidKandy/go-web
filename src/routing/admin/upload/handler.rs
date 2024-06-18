use super::{
    super::super::error::RouterResult,
    db_integration::{save_all_attachments_to_filesystem, TryFromMultipart},
};
use crate::{
    database::{
        handlers::{patch_blog_post, post_blog_post},
        model::{UpdateBlogPost, UploadBlogPost},
    },
    error::{DataApiReturn, IntoDataApiReturn},
    routing::error::RouterError,
    state::AppState,
};
use anyhow::anyhow;
use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    response::{Html, IntoResponse, Response},
};
use serde_json::json;
use std::{ops::Deref, sync::Arc};
use tracing::{error, info, warn};

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
    if let Some(content) = save_all_attachments_to_filesystem(post.attachments.drain(..).collect())
        .map_err(|err| RouterError::from(err).into_data_api_return())?
    {
        post.content = content;
    }
    post_blog_post(&data.db, post).await.map_err(|err| {
        warn!("Encountered an error trying to post: {:?}", err);
        err.into_data_api_return()
    })?;
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
    post.content = save_all_attachments_to_filesystem(post.attachments.drain(..).collect())
        .map_err(|err| {
            error!("error saving attachments! {:?}", err);
            RouterError::from(err).into_data_api_return()
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
