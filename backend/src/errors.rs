use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKS Fetch Error")]
    JWKSFetchError,

    #[display(fmt = "Not Found")]
    NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().body(message.clone())
            }
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().body("Could not fetch JWKS")
            }
            ServiceError::NotFound => HttpResponse::NotFound().body("Not Found"),
        }
    }
}
