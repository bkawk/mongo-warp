extern crate dotenv;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let mongo_user_name = env::var("MONGO_USERNAME").unwrap();
    let mongo_password = env::var("MONGO_PASSWORD").unwrap();
    let mongo_cluster_url = env::var("MONGO_CLUSTER_URL").unwrap();
    let mongo_database = env::var("MONGO_DATABASE").unwrap();
    let mongo_connection_string = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
        mongo_user_name, mongo_password, mongo_cluster_url
    );

    let client_options = ClientOptions::parse(mongo_connection_string).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&mongo_database);

    database.run_command(doc! {"ping": 1}, None).await?;
    println!("Connected successfully.");

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
