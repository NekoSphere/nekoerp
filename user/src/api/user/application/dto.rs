use super::prelude::*;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub trait RepoBounds: Repository + Debug + Default + Send + Sync + 'static {}
impl<T> RepoBounds for T where T: Repository + Debug + Default + Send + Sync + 'static {}
pub trait ServiceBounds: Debug + Default + Send + Sync + 'static {}
impl<T> ServiceBounds for T where T: Debug + Default + Send + Sync + 'static {}

#[derive(Debug)]
pub enum ApplicationError {
    Validation(String),
    Internal(String),
    UserExists,
    WrongEmailOrPassword,
    UserNotFound,
    FirstUserExists,
}

impl From<String> for ApplicationError {
    fn from(s: String) -> Self {
        ApplicationError::Validation(s)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApplicationError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApplicationError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApplicationError::FirstUserExists => (
                StatusCode::BAD_REQUEST,
                "First user already exists in app".to_string(),
            ),
            ApplicationError::WrongEmailOrPassword => (
                StatusCode::BAD_REQUEST,
                "User email or password incorrect".to_string(),
            ),
            ApplicationError::UserExists => (
                StatusCode::BAD_REQUEST,
                "User email exists in app".to_string(),
            ),
            ApplicationError::UserNotFound => {
                (StatusCode::NOT_FOUND, "User not found in app".to_string())
            }
        };

        let body = Json(ErrorBody { error: msg });
        (status, body).into_response()
    }
}
