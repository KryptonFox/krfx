use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordRequestErrorType {
    Name,
    Url,
}

#[derive(Debug, Serialize)]
pub struct RecordRequestError {
    error_type: RecordRequestErrorType,
    message: String,
}

impl RecordRequestError {
    pub fn new(error_type: RecordRequestErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }
}

impl ResponseError for RecordRequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(serde_json::to_string(&self).unwrap())
    }
}

impl Display for RecordRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
