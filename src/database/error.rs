use crate::error::{error_chain_fmt, ErrorResponse, IntoDataApiReturn};
use axum::http::StatusCode;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use tracing::warn;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Undefined(#[from] anyhow::Error),
    Sqlx(sqlx::Error),
    DatabaseConflict,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref err) = value {
            // 23505 is a conflict error
            if let Some("23505") = err.code().as_deref() {
                return Self::DatabaseConflict;
            }
            warn!(
                "Unexpected database error:\n Code: {:?}\n Message: {:?}",
                err.code(),
                err.message()
            );
        }
        Self::Sqlx(value)
    }
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        error_chain_fmt(self, f)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let display = match self {
            Self::Undefined(err) => err.to_string(),
            Self::DatabaseConflict => "Database Conflict".to_string(),
            Self::Sqlx(err) => err.to_string(),
        };

        writeln!(f, "{}", display)
    }
}

impl IntoDataApiReturn for DatabaseError {
    fn status_code(&self) -> StatusCode {
        if let Self::DatabaseConflict = self {
            return StatusCode::CONFLICT;
        }
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> ErrorResponse {
        match self {
            Self::Undefined(err) => ErrorResponse::fail(&format!("undefined error: {:?}", err)),
            Self::Sqlx(err) => ErrorResponse::fail(&format!("Database error: {:?}", err)),
            Self::DatabaseConflict => ErrorResponse::fail("Database conflict"),
        }
    }
}
