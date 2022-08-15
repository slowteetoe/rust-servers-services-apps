use std::fmt;
use sqlx::error::Error as SQLxError;
use actix_web::{http::StatusCode, *};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum EzTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

impl EzTutorError {
    fn error_response(&self) -> String {
        match self {
            EzTutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzTutorError::ActixError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl actix_web::error::ResponseError for EzTutorError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            EzTutorError::ActixError(_msg) | EzTutorError::DBError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzTutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for EzTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzTutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzTutorError {
    fn from(err: SQLxError) -> Self {
        EzTutorError::DBError(err.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
