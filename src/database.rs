use redis;
use redis::AsyncCommands;
use rocket;
use rocket::serde::{json::Json, Deserialize};
use rocket::{Route, State};

use crate::errors::EmptyResult;

#[derive(Deserialize)]
struct Rates<'r> {
    currency: &'r str,
    rate: f32,
}

#[post("/database?<merge>", format = "json", data = "<rates>")]
async fn database(
    merge: bool,
    rates: Json<Vec<Rates<'_>>>,
    redis_client: &State<redis::Client>,
) -> EmptyResult {
    let mut conn = redis_client.get_async_connection().await?;
    if merge {
        let _: () = redis::cmd("FLUSHALL").query_async(&mut conn).await.unwrap();
    }
    for rate in rates.iter() {
        conn.set(rate.currency, rate.rate).await?;
    }
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![database]
}
