use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl ErrorResponse {
    pub fn fail(message: &str) -> Self {
        Self {
            status: "fail",
            message: message.to_string(),
        }
    }

    pub fn success(message: &str) -> Self {
        Self {
            status: "success",
            message: message.to_string(),
        }
    }
}

pub type DataApiReturn = (StatusCode, Json<ErrorResponse>);

pub trait IntoDataApiReturn {
    fn status_code(&self) -> StatusCode;
    fn error_response(&self) -> ErrorResponse;
    fn into_data_api_return(&self) -> DataApiReturn {
        return (self.status_code(), Json(self.error_response()));
    }
}

#[allow(unused_must_use)]
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e);
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
