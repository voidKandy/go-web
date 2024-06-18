use crate::{
    auth::model::FilteredUser,
    database::{
        self,
        handlers::{get_most_recent_posts, get_post_by_title},
        model::{Post, User},
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
use tracing::warn;

#[derive(Template)]
#[template(path = "blog/layout.html")]
pub struct BlogTemplate {
    user: FilteredUser,
    post_opt: Option<PostTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct BlogParams {
    post: Option<String>,
    category: Option<String>,
}

pub struct PostTemplate {
    pub title: String,
    pub subtitle: String,
    pub category: Option<String>,
    pub content: String,
}

pub async fn index(
    Query(params): Query<BlogParams>,
    State(data): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> HandlerResult<Html<String>> {
    let mut post_opt: Option<PostTemplate> = None;
    if let Some(title) = params.post {
        let post = get_post_by_title(&data.db, &title)
            .await
            .map_err(|err| err.status_code())?
            .ok_or_else(|| StatusCode::NOT_FOUND)?;
        post_opt = Some(post.into());
    }
    let tmpl = BlogTemplate {
        user: (&user).into(),
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

impl From<Post> for PostTemplate {
    fn from(value: Post) -> Self {
        Self {
            title: value.title,
            subtitle: value.subtitle.unwrap_or(String::new()),
            category: value.category,
            content: markdown::to_html(&value.content),
        }
    }
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
    let category = params
        .category
        .clone()
        .and_then(|c| Some(c.replace("%20", " ")));
    let posts: Vec<PostTemplate> = get_most_recent_posts(&data.db, category)
        .await
        .map_err(|err| err.status_code())?
        .into_iter()
        .map(|p| PostTemplate::from(p))
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

// pub async fn by_title(
//     Path(encoded_title): Path<String>,
//     State(data): State<Arc<AppState>>,
// ) -> HandlerResult<Html<String>> {
//     let title = encoded_title.replace("%20", " ");
//     let post = get_post_by_title(&data.db, &title)
//         .await
//         .map_err(|err| err.status_code())?
//         .ok_or_else(|| StatusCode::NOT_FOUND)?;
//     let tmpl = PostTemplate::from(post);
//     match tmpl.render() {
//         Ok(r) => Ok(Html(r)),
//         Err(err) => Ok(Html(format!("Error rendering Layout: {}", err.to_string()))),
//     }
//
