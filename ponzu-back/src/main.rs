use crate::endpoints::default::default_responder;
use crate::endpoints::scope::create_app_scope;
use crate::env::get_from_env;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use database::init_database;
use dotenv::dotenv;
use types::app_state::AppState;

mod database;
mod dto;
mod endpoints;
mod env;
mod models;
mod services;
mod types;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    // Load the environment variables
    let db_url: String = get_from_env("DATABASE_URL", None);
    let database: String = get_from_env("DATABASE_NAME", None);
    let port: u16 = get_from_env("PORT", Some("8080"));
    let workers_count: usize = get_from_env("WORKERS", Some("100"));

    // Initialize the app state
    let database = init_database(db_url, database)
        .await
        .expect("Failed to connect to the database");
    let state = web::Data::new(AppState::new(database));

    // Pass the app factory and boot the server
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(create_app_scope())
            .default_service(web::route().to(default_responder))
    })
    .bind(("0.0.0.0", port))?
    .workers(workers_count)
    .run()
    .await
}
