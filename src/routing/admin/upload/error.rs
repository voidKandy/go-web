use axum::http::StatusCode;

use crate::error::{error_chain_fmt, ErrorResponse, IntoDataApiReturn};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type UploadResult<T> = Result<T, UploadError>;
#[derive(thiserror::Error)]
pub enum UploadError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    Multipart(#[from] axum::extract::multipart::MultipartError),
    Io(#[from] std::io::Error),
    Utf8(#[from] core::str::Utf8Error),
}

impl Debug for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Undefined(err) => err.to_string(),
            Self::Multipart(err) => err.to_string(),
            Self::Io(err) => err.to_string(),
            Self::Utf8(err) => err.to_string(),
        };
        writeln!(f, "{}", display)
    }
}

impl IntoDataApiReturn for UploadError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> crate::error::ErrorResponse {
        match self {
            Self::Undefined(err) => {
                ErrorResponse::fail(&format!("An undefined error occurred: {:?}", err))
            }
            Self::Utf8(err) => ErrorResponse::fail(&format!("A UTF8 error occurred: {:?}", err)),
            Self::Io(err) => ErrorResponse::fail(&format!("An IO error occurred: {:?}", err)),
            Self::Multipart(err) => {
                ErrorResponse::fail(&format!("A multipart error occurred: {:?}", err))
            }
        }
    }
}
