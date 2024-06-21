use axum::http::StatusCode;

use crate::error::{error_chain_fmt, ErrorResponse, IntoDataApiReturn};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type BlogResult<T> = Result<T, BlogError>;
#[derive(thiserror::Error)]
pub enum BlogError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    Synect(#[from] syntect::Error),
    Parser(String),
}

impl Debug for BlogError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for BlogError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Undefined(err) => err.to_string(),
            Self::Synect(err) => err.to_string(),
            Self::Parser(err) => err.to_string(),
        };
        writeln!(f, "{}", display)
    }
}

impl IntoDataApiReturn for BlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Undefined(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> crate::error::ErrorResponse {
        match self {
            Self::Undefined(err) => {
                ErrorResponse::fail(&format!("An undefined error occurred: {:?}", err))
            }
            Self::Synect(err) => {
                ErrorResponse::fail(&format!("A syntect error occurred: {:?}", err))
            }
            Self::Parser(err) => {
                ErrorResponse::fail(&format!("A parser error occurred: {:?}", err))
            }
        }
    }
}

impl BlogError {
    pub fn parser_error(message: &str) -> Self {
        Self::Parser(message.to_owned())
    }
}
