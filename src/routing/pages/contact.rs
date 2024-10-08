use askama::Template;
use axum::{response::Html, Form};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate;

type FormData = Vec<(String, String)>;
#[derive(Debug, Deserialize)]
pub struct EmailForm {
    email: String,
    subject: String,
    message: String,
}

impl Into<FormData> for EmailForm {
    fn into(self) -> FormData {
        let web3access_key = std::env::var("WEB3ACCESS_KEY").unwrap();
        vec![
            ("email".to_owned(), self.email),
            ("subject".to_owned(), self.subject),
            ("message".to_owned(), self.message),
            ("access_key".to_owned(), web3access_key),
        ]
    }
}

pub async fn index() -> Html<String> {
    match ContactTemplate.render() {
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
