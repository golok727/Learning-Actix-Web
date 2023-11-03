use actix_web::Error as ActixWebError;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use argon2::Error as Argon2Error;
use core::fmt;
use surrealdb::Error as SurrealDBError;

use serde::Serialize;

const INTERNAL_SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
const BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;

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
    HashError(Option<String>),
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
        //? With messages
        AppError::InternalServerError(Some(msg)) => error_format("InternalServerError", Some(msg)),
        AppError::DatabaseQueryError(Some(msg)) => error_format("DatabaseQueryError", Some(msg)),
        AppError::DatabaseError(Some(msg)) => error_format("DatabaseError", Some(msg)),
        AppError::BadRequest(Some(msg)) => error_format("DatabaseError", Some(msg)),
        AppError::HashError(Some(msg)) => error_format("HashError", Some(msg)),

        //? Without Message
        AppError::InternalServerError(None) => error_format("InternalServerError", None),
        AppError::DatabaseQueryError(None) => error_format("DatabaseQueryError", None),
        AppError::DatabaseError(None) => error_format("DatabaseError", None),
        AppError::BadRequest(None) => error_format("DatabaseError", None),
        AppError::HashError(None) => error_format("HashError", None),
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
            AppError::InternalServerError(None) => INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => INTERNAL_SERVER_ERROR,

            AppError::DatabaseQueryError(None) => INTERNAL_SERVER_ERROR,
            AppError::DatabaseQueryError(Some(_)) => INTERNAL_SERVER_ERROR,

            AppError::DatabaseError(None) => INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => INTERNAL_SERVER_ERROR,

            AppError::HashError(None) => INTERNAL_SERVER_ERROR,
            AppError::HashError(_) => INTERNAL_SERVER_ERROR,

            AppError::BadRequest(None) => BAD_REQUEST,
            AppError::BadRequest(_) => BAD_REQUEST,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = get_error_message(self);

        write!(f, "AppError:\n {}", error_message)
    }
}
impl From<surrealdb::error::Api> for AppError {
    fn from(error: surrealdb::error::Api) -> Self {
        AppError::DatabaseError(Some(format!("{}", error)))
    }
}

impl From<surrealdb::error::Db> for AppError {
    fn from(error: surrealdb::error::Db) -> Self {
        AppError::DatabaseError(Some(format!("{}", error)))
    }
}

impl From<SurrealDBError> for AppError {
    fn from(error: SurrealDBError) -> Self {
        AppError::DatabaseError(Some(format!("{}", error)))
    }
}

impl From<Argon2Error> for AppError {
    fn from(error: Argon2Error) -> Self {
        AppError::InternalServerError(Some(format!("{}", error)))
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::InternalServerError(Some(format!("{}", error)))
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::InternalServerError(Some(format!("{}", error)))
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::InternalServerError(Some(format!("{}", error)))
    }
}

impl From<ActixWebError> for AppError {
    fn from(error: ActixWebError) -> Self {
        AppError::InternalServerError(Some(format!("{}", error)))
    }
}

#[macro_export]
macro_rules! Wrap {
    ($result:expr) => {{
        let mapped: Result<_, AppError> = $result.map_err(AppError::from);
        mapped
    }};

    ($result:expr, $app_error_type:expr) => {{
        let mapped: Result<_, AppError> =
            $result.map_err(|err| $app_error_type(Some(format!("{}", err))));
        mapped
    }};

    ($result:expr, $app_error_type:expr, $message:expr) => {{
        let mapped: Result<_, AppError> =
            $result.map_err(|_| $app_error_type(Some($message.to_string())));
        mapped
    }};
}
