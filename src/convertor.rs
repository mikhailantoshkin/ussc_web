use redis;
use redis::AsyncCommands;
use rocket;
use rocket::serde::{json::Json, Serialize};
use rocket::{Route, State};

use crate::errors::JsonResult;

#[derive(Serialize)]
struct ConversionResponse {
    amount: f32,
}

#[get("/convert?<from>&<to>&<amount>")]
async fn convert(
    from: &str,
    to: &str,
    amount: f32,
    redis_client: &State<redis::Client>,
) -> JsonResult<ConversionResponse> {
    let mut conn = redis_client.get_async_connection().await?;
    let first_rate: f32 = conn.get(from).await?;
    let second_rate: f32 = conn.get(to).await?;
    let converted = first_rate * amount / second_rate;
    Ok(Json(ConversionResponse { amount: converted }))
}

pub fn routes() -> Vec<Route> {
    routes![convert]
}
