use mongodm::mongo::{Client, Database};

pub async fn init_database(
    conn_str: String,
    name: String,
) -> Database {
    // Initialize the database connection
    let client = Client::with_uri_str(conn_str.as_str()).await.unwrap();
    client.database(name.as_str())
}
