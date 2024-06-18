use crate::auth::{middleware::SoftAuthExtension, model::FilteredUser};
use askama::Template;
use axum::{extract::Query, response::Html, Extension};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Template)]
#[template(path = "user/base.html")]
pub struct UserTemplate {
    user: Option<FilteredUser>,
}

#[derive(Template)]
#[template(path = "user/login_register.html")]
pub struct LoginRegisterTemplate {
    login: bool,
}

#[derive(Deserialize)]
pub struct UserTemplateParams {
    mini: bool,
}

pub async fn get_user_template(
    Extension(soft_auth_ext): Extension<SoftAuthExtension>,
) -> Html<String> {
    let template = UserTemplate {
        user: soft_auth_ext.user,
    };
    match template.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}

pub async fn register_user() -> Html<String> {
    let template = LoginRegisterTemplate { login: false };
    match template.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}

pub async fn login_user() -> Html<String> {
    let template = LoginRegisterTemplate { login: true };
    match template.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}
