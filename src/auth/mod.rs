pub mod controller;
mod jwt;
pub mod middleware;
pub mod models;
pub mod service;
pub mod session_repository;

use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("User is not active")]
    UserNotActive,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Database error")]
    DatabaseError,
    #[error("Too many attempts")]
    TooManyAttempts,
    #[error("Session expired")]
    SessionExpired,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            AuthError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => actix_web::http::StatusCode::NOT_FOUND,
            AuthError::UserNotActive => actix_web::http::StatusCode::FORBIDDEN,
            AuthError::TokenCreation => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials => actix_web::http::StatusCode::BAD_REQUEST,
            AuthError::ProviderError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AuthError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::TooManyAttempts => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
            AuthError::SessionExpired => actix_web::http::StatusCode::UNAUTHORIZED,
        };

        HttpResponse::build(status_code).json(json!({
            "error": self.to_string(),
            "code": status_code.as_u16()
        }))
    }
}
