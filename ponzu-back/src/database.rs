use colored::Colorize;
use mongodb::error::Error;
use mongodb::{Client, Database};

/// Initialize the database connection
///
/// # Parameters
/// `conn_str` - The connection string to the database
/// `name` - The name of the database
///
/// # Returns
/// A `Result` containing the `Database` client if successful, or an `Error` if the operation fails.
pub async fn init_database(conn_str: String, name: String)
    -> Result<Database, Error> {
    // Initialize the database connection
    let client = Client::with_uri_str(conn_str).await?;
    let db = client.database(name.as_str());

    println!("{} {}!", "Connected to database", name.bright_green());

    // Return the database client
    Ok(db)
}
