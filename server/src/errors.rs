use redis::{ErrorKind, RedisError};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;

use std::io::Cursor;

#[derive(Debug)]
pub enum ApiError {
    RedisError(RedisError),
}

pub type EmptyResult = Result<(), ApiError>;
pub type JsonResult<T> = Result<Json<T>, ApiError>;

// TODO: return body on error
impl<'r> Responder<'r, 'r> for ApiError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'r> {
        let message = match self {
            ApiError::RedisError(error) => match error.kind() {
                ErrorKind::IoError => return Err(Status::ServiceUnavailable),
                ErrorKind::TypeError => return Err(Status::BadRequest),
                _ => {
                    println!("{:#?}", error);
                    error.to_string()
                }
            },
        };
        Ok(Response::build()
            .status(Status::InternalServerError)
            .sized_body(message.len(), Cursor::new(message))
            .finalize())
    }
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> ApiError {
        ApiError::RedisError(error)
    }
}
