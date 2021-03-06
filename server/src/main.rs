mod convertor;
mod database;
mod errors;

use rocket::Build;

#[macro_use]
extern crate rocket;

fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .manage(redis::Client::open("redis://redis:6379/").unwrap())
        .mount("/", convertor::routes())
        .mount("/", database::routes())
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        drop(e);
    }
}
