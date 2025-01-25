use colored::Colorize;
use mongodb::error::Error;
use mongodb::{Client, Database};

pub async fn init_database(conn_str: String, name: String)
    -> Result<Database, Error> {
    // Initialize the database connection
    let client = Client::with_uri_str(conn_str).await?;
    let db = client.database(name.as_str());

    println!("{} {}!", "Connected to database", name.bright_green());

    // Return the database
    Ok(db)
}
