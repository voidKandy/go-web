use axum::http::StatusCode;

use crate::error::{error_chain_fmt, ErrorResponse, IntoDataApiReturn};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type AuthResult<T> = Result<T, AuthError>;
#[derive(thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    Validation(#[from] uuid::Error),
    JWT(#[from] jsonwebtoken::errors::Error),
    Hash(argon2::password_hash::Error),
    NotLoggedIn,
    NoLongerExists,
}

impl Debug for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Undefined(err) => err.to_string(),
            Self::Hash(err) => err.to_string(),
            Self::NotLoggedIn => "Not Logged In".to_string(),
            Self::NoLongerExists => "No Longer Exists".to_string(),
            Self::Validation(err) => err.to_string(),
            Self::JWT(err) => err.to_string(),
        };
        writeln!(f, "{}", display)
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Hash(value)
    }
}

impl IntoDataApiReturn for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Undefined(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> crate::error::ErrorResponse {
        match self {
            Self::Hash(err) => ErrorResponse::fail(&format!("Hashing error: {:?}", err)),
            Self::NotLoggedIn => ErrorResponse::fail("You are not logged in, please provide token"),
            Self::JWT(err) => ErrorResponse::fail(&format!("Invalid Token: {:?}", err)),
            Self::Validation(err) => ErrorResponse::fail(&format!("Invalid Token: {:?}", err)),
            Self::NoLongerExists => ErrorResponse::fail(&"The user no longer exists".to_string()),
            Self::Undefined(err) => {
                ErrorResponse::fail(&format!("An undefined error occurred: {:?}", err))
            }
        }
    }
}
