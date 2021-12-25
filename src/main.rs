mod convertor;
mod database;

use rocket::{Build, State};

#[macro_use]
extern crate rocket;

fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .manage(redis::Client::open("redis://localhost:6379/").unwrap())
        .mount("/", convertor::routes())
        .mount("/", database::routes())
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        drop(e);
    }
}
