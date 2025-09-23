use anyhow::anyhow;
use axum::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalServerError(anyhow::Error),
}

impl AppError {
    fn not_found<T>(t: T) -> Self
    where
        T: ToString,
    {
        AppError::NotFound(t.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err).into_response(),
            AppError::InternalServerError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
        }
    }
}
