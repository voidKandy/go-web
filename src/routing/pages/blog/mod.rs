pub mod error;
pub mod post;
use crate::{
    auth::{middleware::SoftAuthExtension, model::FilteredUser},
    database::{
        self,
        handlers::{get_most_recent_posts, get_post_by_title},
    },
    error::IntoDataApiReturn,
    routing::HandlerResult,
    state::AppState,
};
use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    Extension,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, warn};

use self::post::PostTemplate;

#[derive(Template)]
#[template(path = "blog/layout.html")]
pub struct BlogTemplate {
    user_opt: Option<FilteredUser>,
    post_opt: Option<PostTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct BlogParams {
    post: Option<String>,
    category: Option<String>,
}

pub async fn index(
    Query(params): Query<BlogParams>,
    State(data): State<Arc<AppState>>,
    Extension(soft_auth_ext): Extension<SoftAuthExtension>,
) -> HandlerResult<Html<String>> {
    let mut post_opt: Option<PostTemplate> = None;
    if let Some(title) = params.post {
        let post = get_post_by_title(&data.db, &title)
            .await
            .map_err(|err| err.status_code())?
            .ok_or_else(|| StatusCode::NOT_FOUND)?;
        post_opt = Some(
            post.try_into()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        );
    }
    let tmpl = BlogTemplate {
        user_opt: soft_auth_ext.user,
        post_opt,
    };
    match tmpl.render() {
        Ok(r) => Ok(Html(r)),
        Err(err) => Ok(Html(format!("Error rendering Layout: {}", err.to_string()))),
    }
}
#[derive(Template)]
#[template(path = "blog/multi-post.html")]
pub struct MultiPostTemplate {
    posts: Vec<PostTemplate>,
    categories: Vec<String>,
    selected_category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LatestParams {
    category: Option<String>,
}

mod filters {
    // removes spaces
    pub fn sanitize_whitespace_filter<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(s.to_string().replace(" ", "%20"))
    }
}

pub async fn latest(
    State(data): State<Arc<AppState>>,
    Query(params): Query<LatestParams>,
) -> HandlerResult<Html<String>> {
    let categories = database::handlers::get_blog_categories(&data.db)
        .await
        .map_err(|err| {
            warn!("error getting categories: {:?}", err);
            err.status_code()
        })?;
    debug!("got categories: {:?}", categories);
    let category = params
        .category
        .clone()
        .and_then(|c| Some(c.replace("%20", " ")));
    let posts: Vec<PostTemplate> = get_most_recent_posts(&data.db, category)
        .await
        .map_err(|err| err.status_code())?
        .into_iter()
        .map(|p| {
            debug!("processing: {:?}", p);
            PostTemplate::try_from(p).expect("failed to create post template")
        })
        .collect();
    let tmpl = MultiPostTemplate {
        posts,
        categories,
        selected_category: params.category,
    };
    match tmpl.render() {
        Ok(r) => Ok(Html(r)),
        Err(err) => Ok(Html(format!("Error rendering Layout: {}", err.to_string()))),
    }
}
