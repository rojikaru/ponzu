use crate::models::error_response::SerializableError;
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
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::HttpError(msg, status_code) => {
                write!(f, "HTTP error: {} ({})", msg, status_code.to_string())
            }
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::MongoError(_) => HttpResponse::InternalServerError()
                .json(r#"{"error": "Database error", "status": 500}"#),
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(SerializableError::new(msg.to_string(), 404))
            }
            AppError::InternalServerError(msg) => HttpResponse::InternalServerError()
                .json(SerializableError::new(msg.to_string(), 500)),
            AppError::HttpError(msg, status_code) => HttpResponse::build(*status_code).json(
                SerializableError::new(msg.to_string(), status_code.as_u16()),
            ),
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

impl From<(String, u16)> for AppError {
    fn from((msg, status_code): (String, u16)) -> Self {
        AppError::HttpError(msg, StatusCode::from_u16(status_code).unwrap())
    }
}

impl From<(&str, u16)> for AppError {
    fn from((msg, status_code): (&str, u16)) -> Self {
        AppError::HttpError(msg.to_owned(), StatusCode::from_u16(status_code).unwrap())
    }
}
