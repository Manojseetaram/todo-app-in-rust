use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;

pub async fn connect_to_db() -> Database {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env");

    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");

    client.database("todo_db")
}
