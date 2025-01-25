use colored::Colorize;
use mongodm::mongo::{Client, Database};

pub async fn init_database(conn_str: String, name: String) -> Database {
    // Initialize the database connection
    let client = Client::with_uri_str(conn_str).await.unwrap();
    let db = client.database(name.as_str());

    println!("{} {}!", "Connected to database", name.bright_green());

    // Return the database
    db
}
