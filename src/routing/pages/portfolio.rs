use askama::Template;
use axum::{extract::Path, response::Html};
use reqwest::StatusCode;
use std::fs;
use tracing::error;

use crate::routing::HandlerResult;

#[derive(Template)]
#[template(path = "portfolio/layout.html")]
pub struct PortfolioTemplate;

#[derive(Template)]
#[template(path = "portfolio/item.html")]
pub struct PortfolioItemTemplate {
    pub title: String,
    pub subtitle: String,
    pub content: String,
}

fn deathwish_tmpl() -> anyhow::Result<PortfolioItemTemplate> {
    let content = fs::read_to_string("public/assets/portfolio/deathwish.md")?;
    let content = markdown::to_html(&content);
    Ok(PortfolioItemTemplate {
        title: "Deathwish Powersports".to_string(),
        subtitle: "Freelance Full Stack Development".to_string(),
        content,
    })
}

fn espionox_tmpl() -> anyhow::Result<PortfolioItemTemplate> {
    let content = fs::read_to_string("public/assets/portfolio/espionox.md")?;
    let content = markdown::to_html(&content);
    Ok(PortfolioItemTemplate {
        title: "Espionox".to_string(),
        subtitle: "A Rust crate for building LLM Applications".to_string(),
        content,
    })
}

fn lovetogether_tmpl() -> anyhow::Result<PortfolioItemTemplate> {
    let content = fs::read_to_string("public/assets/portfolio/lovetogether.md")?;
    let content = markdown::to_html(&content);

    Ok(PortfolioItemTemplate {
        title: "LoveTogether".to_string(),
        subtitle: "Full Stack Intern".to_string(),
        content,
    })
}

fn prattl_tmpl() -> anyhow::Result<PortfolioItemTemplate> {
    let content = fs::read_to_string("public/assets/portfolio/prattl.md")?;
    let content = markdown::to_html(&content);

    Ok(PortfolioItemTemplate {
        title: "Prattl".to_string(),
        subtitle: "Local Transcription CLI tool".to_string(),
        content,
    })
}

pub async fn index(Path(work_type): Path<String>) -> HandlerResult<Html<String>> {
    match match work_type.as_str() {
        "deathwish" => deathwish_tmpl()
            .map_err(|err| {
                error!("there was an error getting a portfolio template: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .render(),
        "espionox" => espionox_tmpl()
            .map_err(|err| {
                error!("there was an error getting a portfolio template: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .render(),
        "lovetogether" => lovetogether_tmpl()
            .map_err(|err| {
                error!("there was an error getting a portfolio template: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .render(),
        "prattl" => prattl_tmpl()
            .map_err(|err| {
                error!("there was an error getting a portfolio template: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .render(),
        _ => return Err(StatusCode::NOT_FOUND),
    } {
        Ok(r) => Ok(Html(r)),
        Err(err) => Ok(Html(format!(
            "Error rendering template: {}",
            err.to_string()
        ))),
    }
}
