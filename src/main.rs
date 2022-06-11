mod auth;

use dotenv;
use tokio;
use std::{env, error::Error};
use mongodb::{options::{ClientOptions}, Client, bson::doc};
use warp::Filter;

use crate::auth::auth_filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI .env");
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;
    let users = client.database("rusty_db").collection("users");
    let user = doc! { "password": "1984", "username": "GeorgeOrwell" };
    let insert_result = users.insert_one(user, None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    let apis = auth_filter();
    let welcome = warp::path::end().map(|| "Welcome to my api");

    let routes = apis.or(welcome);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}


// https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
// https://www.youtube.com/watch?v=HNnbIW2Kzbc