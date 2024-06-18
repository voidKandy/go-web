use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;

pub async fn index() -> Html<String> {
    match LandingTemplate.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}
