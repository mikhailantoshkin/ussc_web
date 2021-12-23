use rocket;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::{status, Debug};
use rocket::State;

use redis;
use redis::AsyncCommands;
use rocket::response::status::NotFound;

type MyResult<T, E = Debug<redis::RedisError>> = std::result::Result<T, E>;

#[get("/<name>")]
async fn index(
    name: &str,
    redis_client: &State<redis::Client>,
) -> Result<String, status::Custom<String>> {
    let mut conn = redis_client
        .get_async_connection()
        .await
        .map_err(|e| status::Custom(Status::ServiceUnavailable, "Dead redis".to_string()))?;
    let res: String = conn
        .get("test")
        .await
        .map_err(|e| status::Custom(Status::BadRequest, e.to_string()))?;
    let _: () = conn.set("test", name).await.unwrap();
    // "Hello, world!"
    Ok(format!("Hello, {}", res))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Converters handlers", |rocket| async {
        rocket
            .manage(redis::Client::open("redis://localhost:6379/").unwrap())
            .mount("/", routes![index])
    })
}
