use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use redis::{ErrorKind, RedisError};

#[derive(Debug)]
pub enum ApiError {
    RedisError(RedisError),
}

pub type EmptyResult = Result<(), ApiError>;
pub type JsonResult<T> = Result<Json<T>, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let resp = match self {
            ApiError::RedisError(error) => match error.kind() {
                ErrorKind::IoError => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Redis is unavailable".to_string(),
                ),
                ErrorKind::TypeError => (StatusCode::BAD_REQUEST, "Wrong data format".to_string()),
                _ => {
                    println!("{:#?}", error);
                    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
                }
            },
        };
        resp.into_response()
    }
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> ApiError {
        ApiError::RedisError(error)
    }
}
