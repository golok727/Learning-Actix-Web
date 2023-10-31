use core::fmt;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct AppErrorResponse {
    status: u16,
    message: String,
}
impl AppErrorResponse {
    pub fn new(status: u16, message: String) -> Self {
        AppErrorResponse { status, message }
    }
}

#[derive(Debug)]
pub enum AppError {
    InternalServerError(Option<String>),
    DatabaseQueryError(Option<String>),
    DatabaseError(Option<String>),
    BadRequest(Option<String>),
}
fn error_format<'a>(error_type: &'a str, message: Option<&'a str>) -> String {
    match message {
        Some(msg) => {
            format!("Error: {}\n Message: {}", error_type, msg)
        }
        _ => format!("Error: {}", error_type),
    }
}

fn get_error_message(error: &AppError) -> String {
    match error {
        AppError::InternalServerError(Some(msg)) => error_format("InternalServerError", Some(msg)),
        AppError::DatabaseQueryError(Some(msg)) => error_format("DatabaseQueryError", Some(msg)),
        AppError::DatabaseError(Some(msg)) => error_format("DatabaseError", Some(msg)),

        AppError::BadRequest(Some(msg)) => error_format("DatabaseError", Some(msg)),

        AppError::InternalServerError(None) => error_format("InternalServerError", None),
        AppError::DatabaseQueryError(None) => error_format("DatabaseQueryError", None),
        AppError::DatabaseError(None) => error_format("DatabaseError", None),
        AppError::BadRequest(None) => error_format("DatabaseError", None),
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_message = get_error_message(self);

        let status_code = self.status_code();

        let response = AppErrorResponse::new(status_code.as_u16(), error_message);

        HttpResponse::build(status_code)
            .content_type(ContentType::json())
            .json(response)
    }

    // Override the status code function
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(None) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::DatabaseQueryError(None) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseQueryError(Some(_)) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::DatabaseError(None) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            AppError::BadRequest(None) => StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = get_error_message(self);

        write!(f, "AppError:\n {}", error_message)
    }
}
