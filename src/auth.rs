use serde_json::{json, Value};
use warp::{Filter, reply::Json};

pub fn auth_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let join = warp::path("join")
    .and(warp::get())
    .and(warp::path::end())
    .and_then(join_handler);

    let signin = warp::path("signin")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(signin_handler);

    join.or(signin)
}

async fn join_handler() -> Result<Json, warp::Rejection> {
    let user = json!({"username":"GeorgeOrwell", "password": "1984", "id": "62a5108336aaebf431faa522"});
    let user = warp::reply::json(&user);
    Ok(user)
}

async fn signin_handler(data: Value) -> Result<Json, warp::Rejection> {
    let credentials = data;
    let credentials = warp::reply::json(&credentials);
    Ok(credentials)
}