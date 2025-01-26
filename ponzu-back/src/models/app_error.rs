use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use mongodb::error::Error as MongoError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    MongoError(MongoError),
    NotFound(String),
    InternalServerError(String),
    HttpError(String, StatusCode),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MongoError(err) => write!(f, "MongoDB error: {}", err),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InternalServerError(msg) =>
                write!(f, "Internal server error: {}", msg),
            AppError::HttpError(msg, status_code) => {
                write!(f, "HTTP error: {} ({})", msg, status_code.to_string())
            }
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::MongoError(_) => HttpResponse::InternalServerError().json("Database error"),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            AppError::InternalServerError(msg) => HttpResponse::InternalServerError().json(msg),
            AppError::HttpError(msg, status_code) => HttpResponse::build(*status_code).json(msg),
        }
    }
}

impl From<MongoError> for AppError {
    fn from(err: MongoError) -> Self {
        AppError::MongoError(err)
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::InternalServerError(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::InternalServerError(msg.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<(String, StatusCode)> for AppError {
    fn from((msg, status_code): (String, StatusCode)) -> Self {
        AppError::HttpError(msg, status_code)
    }
}
