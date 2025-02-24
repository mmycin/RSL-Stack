use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl ApiError {
    pub fn to_response(&self) -> status::Custom<Value> {
        match self {
            ApiError::DatabaseError(e) => status::Custom(
                Status::InternalServerError,
                json!({"error": format!("Database error: {}", e)}),
            ),
            ApiError::NotFound(msg) => status::Custom(
                Status::NotFound,
                json!({"error": msg}),
            ),
            ApiError::BadRequest(msg) => status::Custom(
                Status::BadRequest,
                json!({"error": msg}),
            ),
            ApiError::InternalServerError(msg) => status::Custom(
                Status::InternalServerError,
                json!({"error": msg}),
            ),
        }
    }
}