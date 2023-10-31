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
pub enum AppError<'a> {
    InternalServerError(Option<&'a str>),
}

impl ResponseError for AppError<'_> {
    fn error_response(&self) -> HttpResponse {
        let error_message = match self {
            AppError::InternalServerError(Some(msg)) => msg.to_string(),
            AppError::InternalServerError(None) => "Internal Server Error".to_string(),
        };

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
        }
    }
}

impl fmt::Display for AppError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = match self {
            AppError::InternalServerError(Some(msg)) => msg.to_string(),
            AppError::InternalServerError(None) => "Internal Error".to_string(),
        };

        write!(f, "Error: {}", error_message)
    }
}
