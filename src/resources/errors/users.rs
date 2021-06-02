use actix_web::http::StatusCode;

use crate::resources::errors::ErrorKind;

#[derive(Debug)]
pub enum UsersError {
    NotFound,
    Error(String),
}

impl ErrorKind for UsersError {
    fn code(&self) -> u16 {
        match self {
            UsersError::NotFound => 10,
            _ => 0,
        }
    }

    fn message(&self) -> String {
        match self {
            UsersError::NotFound => r"User Not Found Or Inactive".into(),
            _ => r"Internal Server Error".into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UsersError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn report(&self) -> Option<String> {
        match self {
            UsersError::NotFound => None,
            UsersError::Error(report) => Some(report.to_owned()),
        }
    }
}
