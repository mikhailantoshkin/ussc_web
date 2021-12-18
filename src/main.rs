use redis;
use rocket::State;
// fn connect_to_to_redis() -> redis::Connection {
//    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
// }
#[macro_use]
extern crate rocket;

#[get("/")]
async fn index(redis_cliednt: &State<redis::Client>) -> &'static str {
    let conn = redis_cliednt.get_async_connection().await.unwrap();
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(redis::Client::open("redis://redis/").unwrap())
        .mount("/", routes![index])
}
