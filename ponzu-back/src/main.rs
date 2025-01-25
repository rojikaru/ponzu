use crate::env::get_from_env;
use crate::services::app_state::AppState;
use crate::services::database::init_database;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod env;
mod services;

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
    let state = web::Data::new(AppState::new(init_database(db_url, database).await));

    // Pass the app factory and boot the server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api"))
    })
    .bind(("0.0.0.0", port))?
    .workers(workers_count)
    .run()
    .await
}
