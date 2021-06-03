use actix_web::http::StatusCode;

use crate::resources::errors::ErrorKind;

#[derive(Debug)]
pub enum AuthError {
    InvalidRequest(String),
    Unauthorized(u16, Option<String>),
    Forbidden(u16, Option<String>),
}

impl ErrorKind for AuthError {
    fn code(&self) -> u16 {
        match self {
            AuthError::InvalidRequest(_) => 1,

            AuthError::Unauthorized(code, _) => code.to_owned(),
            AuthError::Forbidden(code, _) => code.to_owned(),
        }
    }

    fn message(&self) -> String {
        match self {
            AuthError::InvalidRequest(_) => r"Invalid body request, see the documentation.".into(),
            AuthError::Unauthorized(_, Some(message)) => message.to_owned(),
            AuthError::Forbidden(_, Some(message)) => message.to_owned(),
            _ => r"Access denied".into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AuthError::Forbidden(_, _) => StatusCode::FORBIDDEN,
            AuthError::Unauthorized(_, _) => StatusCode::UNAUTHORIZED,
        }
    }

    fn report(&self) -> Option<String> {
        match self {
            AuthError::InvalidRequest(report) => Some(report.to_owned()),
            _ => None,
        }
    }
}
