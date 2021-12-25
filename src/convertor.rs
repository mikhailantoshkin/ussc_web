use rocket;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::{status, Debug};
use rocket::{Route, State};

use redis;
use redis::AsyncCommands;
use rocket::response::status::NotFound;

#[get("/convert?<from>&<to>&<amount>")]
async fn convert(
    from: &str,
    to: &str,
    amount: f32,
    redis_client: &State<redis::Client>,
) -> Result<String, status::Custom<String>> {
    let mut conn = redis_client
        .get_async_connection()
        .await
        .map_err(|e| status::Custom(Status::ServiceUnavailable, "Dead redis".to_string()))?;
    let first_rate: f32 = conn
        .get(from)
        .await
        .map_err(|e| status::Custom(Status::BadRequest, e.to_string()))?;
    let second_rate: f32 = conn
        .get(to)
        .await
        .map_err(|e| status::Custom(Status::BadRequest, e.to_string()))?;
    let converted = first_rate * amount / second_rate;
    Ok(converted.to_string())
}

pub fn routes() -> Vec<Route> {
    routes![convert]
}
