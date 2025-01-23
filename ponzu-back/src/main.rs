mod endpoints;
mod env;

use crate::endpoints::{echo, hello, manual_hello};
use crate::env::get_from_env;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    // Load the environment variables
    let port: u16 = get_from_env("PORT", Some("8080"));
    let workers: usize = get_from_env("WORKERS", Some("100"));

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", port))?
    .workers(workers)
    .run()
    .await
}
