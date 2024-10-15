use anyhow::anyhow;
use askama::Template;
use axum::{extract::Path, response::Html};
use core::panic;
use reqwest::StatusCode;
use tracing::warn;

use crate::{
    hilighting::{Content, HighlighterParser},
    routing::HandlerResult,
};

// #[derive(Template)]
// #[template(path = "portfolio/layout.html")]
// pub struct PortfolioTemplate;
#[derive(Template, Debug)]
#[template(path = "portfolio/item.html")]
pub struct PortfolioItemTemplate {
    pub title: String,
    pub subtitle: String,
    pub content: Vec<String>,
}

const PORTFOLIO_DIR: &str = "public/assets/portfolio/";
const DEATHWISH: &str = "deathwish";
const ESPIONOX: &str = "espionox";
const PRATTL: &str = "prattl";
const LOVETOGETHER: &str = "love-together";
const SEMPERFLIES: &str = "semperflies";

pub const ALL_PORTFOLIO_ITEMS: [&str; 5] = [DEATHWISH, ESPIONOX, PRATTL, LOVETOGETHER, SEMPERFLIES];

impl PortfolioItemTemplate {
    pub fn to_display(str: &str) -> String {
        let mut buffer = String::new();
        let mut capitalize = true;
        for char in str.chars() {
            if capitalize {
                buffer.push_str(&char.to_uppercase().to_string());
                capitalize = false;
            } else {
                if char == '-' {
                    capitalize = true;
                    buffer.push(' ');
                } else {
                    buffer.push(char);
                }
            }
        }
        buffer
    }
}

fn item_template(str: &str) -> anyhow::Result<PortfolioItemTemplate> {
    let filepath = format!("{PORTFOLIO_DIR}{str}.md");
    let content = std::fs::read_to_string(filepath)?;
    let info = content.splitn(3, "---").collect::<Vec<&str>>();
    if !info[0].trim().is_empty() {
        return Err(anyhow!("expected empty string, got: {:?}", info[0]));
    }

    let (title, subtitle) = parse_subtitle_and_title_from_content(info[1]);

    let mut parser = HighlighterParser::new(&info[2]);
    let content = parser
        .highlight_code_blocks()?
        .into_iter()
        .map(|c| c.into_html().expect("Failed to convert into html"))
        .collect();

    Ok(PortfolioItemTemplate {
        title,
        subtitle,
        content,
    })
}

fn parse_subtitle_and_title_from_content(str: &str) -> (String, String) {
    warn!("parsing for title and subtitle from: {str}");
    let mut title = String::new();
    let mut subtitle = String::new();
    for line in str.lines() {
        if line.contains("subtitle: ") {
            if let Some(split) = line.split_once("subtitle: ") {
                subtitle = split.1.trim().to_string();
            }
        } else {
            if let Some(split) = line.split_once("title: ") {
                title = split.1.trim().to_string();
            }
        }
    }

    return (title, subtitle);
}

pub async fn index(Path(work_type): Path<String>) -> HandlerResult<Html<String>> {
    let template = item_template(&work_type).map_err(|err| {
        warn!(
            "there was an error getting a portfolio template for {work_type}: {:?}",
            err
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    warn!("got item template: {template:#?}");

    match template.render() {
        Ok(r) => Ok(Html(r)),
        Err(err) => Ok(Html(format!(
            "Error rendering template: {}",
            err.to_string()
        ))),
    }
}

mod tests {
    use super::parse_subtitle_and_title_from_content;

    #[test]
    fn title_and_subtitle() {
        let input = r#"
        title: title
        subtitle: subtitle
        "#;

        let (title, subtitle) = parse_subtitle_and_title_from_content(input);

        assert_eq!(&title, "title");
        assert_eq!(&subtitle, "subtitle");
    }
}
