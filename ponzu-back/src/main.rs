mod endpoints;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::endpoints::{echo, hello, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok();

    // Load the environment variables
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}


