mod convertor;

use rocket::{Build, State};

#[macro_use]
extern crate rocket;

fn rocket() -> rocket::Rocket<Build> {
    rocket::build().attach(convertor::stage())
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        drop(e);
    }
}
