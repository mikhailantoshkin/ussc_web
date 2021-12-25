use rocket;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::{status, Debug};
use rocket::{Route, State};

use redis;
use redis::aio::ConnectionLike;
use redis::AsyncCommands;
use rocket::response::status::NotFound;
use rocket::serde::{json::Json, Deserialize};

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
) -> Result<(), status::Custom<String>> {
    let mut conn = redis_client
        .get_async_connection()
        .await
        .map_err(|e| status::Custom(Status::ServiceUnavailable, "Dead redis".to_string()))?;
    if merge {
        let _: () = redis::cmd("FLUSHALL").query_async(&mut conn).await.unwrap();
    }
    for rate in rates.iter() {
        conn.set(rate.currency, rate.rate)
            .await
            .map_err(|e| status::Custom(Status::BadRequest, e.to_string()))?;
    }
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![database]
}
