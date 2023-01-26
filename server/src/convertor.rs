use std::sync::Arc;

use crate::errors::JsonResult;
use axum::{
    extract::{Query, State},
    Json,
};
use redis::Client;
use redis::Commands;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CurrencyQuery {
    from: String,
    to: String,
    amount: f32,
}

#[derive(Serialize)]
pub struct CurrencyResp {
    result: f32,
}

pub async fn convert(
    params: Query<CurrencyQuery>,
    State(pool): State<Arc<Client>>,
) -> JsonResult<CurrencyResp> {
    let from = &params.from;
    let to = &params.to;
    let amount = &params.amount;

    let mut conn = pool.get_connection()?;
    let first_rate: f32 = conn.get(from)?;
    let second_rate: f32 = conn.get(to)?;
    let converted = first_rate * amount / second_rate;
    Ok(Json(CurrencyResp { result: converted }))
}
