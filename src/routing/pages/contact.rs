use askama::Template;
use axum::{response::Html, Extension, Form};
use serde::Deserialize;

use crate::auth::{middleware::SoftAuthExtension, model::FilteredUser};

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    user: Option<FilteredUser>,
}

type FormData = Vec<(String, String)>;
#[derive(Debug, Deserialize)]
pub struct EmailForm {
    email: String,
    subject: String,
    message: String,
}
const WEB3ACCESS_KEY: &'static str = "145d9a7c-79aa-40b7-bb08-9d75c3f333db";

impl Into<FormData> for EmailForm {
    fn into(self) -> FormData {
        vec![
            ("email".to_owned(), self.email),
            ("subject".to_owned(), self.subject),
            ("message".to_owned(), self.message),
            ("access_key".to_owned(), WEB3ACCESS_KEY.to_string()),
        ]
    }
}

pub async fn index(Extension(soft_auth_ext): Extension<SoftAuthExtension>) -> Html<String> {
    let tmpl = ContactTemplate {
        user: soft_auth_ext.user,
    };
    match tmpl.render() {
        Ok(r) => Html(r),
        Err(err) => Html(format!("Error rendering Layout: {}", err.to_string())),
    }
}

pub async fn send_email(Form(email_form): Form<EmailForm>) -> Html<String> {
    let web3form_url = "https://api.web3forms.com/submit";
    let client = reqwest::Client::new();
    let response = match client
        .post(web3form_url)
        .form(&Into::<FormData>::into(email_form))
        .send()
        .await
    {
        Ok(_) => "<p>Email sent successfully</p>".to_owned(),
        Err(err) => format!("<p x-style='color:red;'>Error sending email: {:?}</p>", err),
    };

    Html(response)
}
