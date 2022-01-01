use async_graphql::ErrorExtensions;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OwdaError {
    #[error("Could not find resource")]
    NotFound,
    #[error("Operation not allowed")]
    Unauthorized,
    #[error("Operation failed")]
    Forbidden,
    #[error("Invalid input")]
    BadRequest,
    #[error("Internal server error")]
    InternalServerError,
}

impl ErrorExtensions for OwdaError {
    fn extend(&self) -> async_graphql::Error {
        self.extend_with(|_, e| match self {
            OwdaError::NotFound => e.set("code", StatusCode::NOT_FOUND.as_str()),
            OwdaError::Unauthorized => e.set("code", StatusCode::UNAUTHORIZED.as_str()),
            OwdaError::Forbidden => e.set("code", StatusCode::FORBIDDEN.as_str()),
            OwdaError::BadRequest => e.set("code", StatusCode::BAD_REQUEST.as_str()),
            OwdaError::InternalServerError => {
                e.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_str())
            }
        })
    }
}
