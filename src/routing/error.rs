use axum::http::StatusCode;

use crate::error::{error_chain_fmt, ErrorResponse, IntoDataApiReturn};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type RouterResult<T> = Result<T, RouterError>;
#[derive(thiserror::Error)]
pub enum RouterError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    Template(#[from] askama::Error),
    // Multipart(#[from] axum::extract::multipart::MultipartError),
    // Utf8(#[from] core::str::Utf8Error),
}

impl Debug for RouterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for RouterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Undefined(err) => err.to_string(),
            Self::Template(err) => err.to_string(),
            // Self::Multipart(err) => err.to_string(),
            // Self::Utf8(err) => err.to_string(),
        };
        writeln!(f, "{}", display)
    }
}

impl IntoDataApiReturn for RouterError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> crate::error::ErrorResponse {
        match self {
            Self::Template(err) => {
                ErrorResponse::fail(&format!("An askama error occurred: {:?}", err))
            }
            Self::Undefined(err) => {
                ErrorResponse::fail(&format!("An undefined error occurred: {:?}", err))
            } // Self::Utf8(err) => ErrorResponse::fail(&format!("A UTF8 error occurred: {:?}", err)),
              // Self::Multipart(err) => {
              //     ErrorResponse::fail(&format!("A multipart error occurred: {:?}", err))
              // }
        }
    }
}
