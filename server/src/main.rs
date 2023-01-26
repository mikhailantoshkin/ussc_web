use std::sync::Arc;

use crate::convertor::convert;
use crate::database::database;
use axum::routing::{get, post};
use axum::Router;

mod convertor;
mod database;
mod errors;

#[tokio::main]
async fn main() {
    let pool = Arc::new(redis::Client::open("redis://redis:6379/").unwrap());
    let app = Router::new()
        .route("/convert", get(convert))
        .route("/database", post(database))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
