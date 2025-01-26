use crate::models::error_response::SerializableError;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn default_responder(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json(
        SerializableError::new(
            format!("Resource {} not found", req.path()),
            404
        )
    )
}
