use actix_web::{web, Scope};
use crate::endpoints::anime::title::create_anime_scope;

pub fn create_app_scope() -> Scope {
    web::scope("/api")
        .service(create_anime_scope())
}
