use actix_web::http::StatusCode;

use crate::resources::errors::ErrorKind;

#[derive(Debug)]
pub enum AuthError {
    InvalidRequest(String),
    TokenNotFound,
    InvalidToken,
    Unauthorized,
}

impl ErrorKind for AuthError {
    fn code(&self) -> u16 {
        match self {
            AuthError::InvalidRequest(_) => 1,
            AuthError::TokenNotFound => 2,
            AuthError::InvalidToken => 3,
            AuthError::Unauthorized => 4,
        }
    }

    fn message(&self) -> String {
        match self {
            AuthError::InvalidRequest(_) => r"Invalid body request, see the documentation.".into(),
            AuthError::TokenNotFound => r"Token must be provided".into(),
            AuthError::InvalidToken => r"Token provided isn't valid".into(),
            AuthError::Unauthorized => r"Access denied".into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AuthError::TokenNotFound => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn report(&self) -> Option<String> {
        match self {
            AuthError::InvalidRequest(report) => Some(report.to_owned()),
            _ => None,
        }
    }
}
