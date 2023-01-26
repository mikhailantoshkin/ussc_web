use std::sync::Arc;

use crate::errors::EmptyResult;
use axum::{
    extract::{Query, State},
    Json,
};
use redis::{Client, AsyncCommands};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseQuery {
    merge: bool,
}

#[derive(Deserialize)]
pub struct Rates {
    currency: String,
    rate: f32,
}

pub async fn database(
    params: Query<DatabaseQuery>,
    State(pool): State<Arc<Client>>,
    Json(payload): Json<Vec<Rates>>,
) -> EmptyResult {
    let mut conn = pool.get_async_connection().await?;

    if !params.merge {
        let _: () = redis::cmd("FLUSHALL").query_async(&mut conn).await?;
    }
    let data: Vec<(String, f32)> = payload.into_iter().map(|rate| (rate.currency, rate.rate)).collect();
    let _: () = conn.set_multiple(&data).await?;
    Ok(())
}
