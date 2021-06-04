use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::{error::Error, fmt::Display};

pub mod auth;
pub mod users;
pub mod validation;

#[derive(Debug, serde::Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
    #[cfg_attr(not(debug_assertions), serde(skip))]
    pub report: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct ErrorBody {
    error: ErrorMessage,
}

pub trait ErrorKind: Sized {
    fn code(&self) -> u16;
    fn message(&self) -> String;

    fn report(&self) -> Option<String> {
        None
    }

    fn from_none() -> Option<Self> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum ServiceError<K>
where
    K: ErrorKind,
{
    BadRequest(K),   // 400
    Unauthorized(K), // 401
    Forbidden(K),    // 403
    #[allow(dead_code)]
    NotFound(K),
    #[allow(dead_code)]
    PreconditionFailed(K), // 412
    InternalServerError(K), // 500
}

impl<K> ServiceError<K>
where
    K: ErrorKind,
{
    pub fn bad_request(kind: K) -> Self {
        Self::BadRequest(kind)
    }

    pub fn unauthorized(kind: K) -> Self {
        Self::Unauthorized(kind)
    }

    pub fn forbidden(kind: K) -> Self {
        Self::Forbidden(kind)
    }

    #[allow(dead_code)]
    pub fn not_found(kind: K) -> Self {
        Self::NotFound(kind)
    }

    #[allow(dead_code)]
    pub fn precondition_failed(kind: K) -> Self {
        Self::PreconditionFailed(kind)
    }

    pub fn internal(kind: K) -> Self {
        Self::InternalServerError(kind)
    }

    fn kind(&self) -> &K {
        match self {
            Self::BadRequest(kind) => kind,
            Self::Unauthorized(kind) => kind,
            Self::Forbidden(kind) => kind,
            Self::NotFound(kind) => kind,
            Self::PreconditionFailed(kind) => kind,
            Self::InternalServerError(kind) => kind,
        }
    }

    fn get_error_body(&self) -> ErrorBody {
        let kind = self.kind();

        let message = if let Self::InternalServerError(_) = self {
            r"Internal Server Error".to_owned()
        } else {
            kind.message()
        };

        let error = ErrorMessage {
            code: kind.code(),
            message,
            report: kind.report(),
        };

        ErrorBody { error }
    }
}

impl<K: ErrorKind> std::fmt::Display for ServiceError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_error_body().error)
    }
}

impl<K: ErrorKind + std::fmt::Debug> ResponseError for ServiceError<K> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut builder = match self {
            Self::BadRequest(_) => HttpResponse::BadGateway(),
            Self::Unauthorized(_) => HttpResponse::Unauthorized(),
            Self::Forbidden(_) => HttpResponse::Forbidden(),
            Self::NotFound(_) => HttpResponse::NotFound(),
            Self::PreconditionFailed(_) => HttpResponse::PreconditionFailed(),
            Self::InternalServerError(_) => HttpResponse::InternalServerError(),
        };

        builder.json(self.get_error_body())
    }
}

impl<K: std::error::Error> ErrorKind for K {
    fn code(&self) -> u16 {
        0
    }

    fn message(&self) -> String {
        "".into()
    }

    fn report(&self) -> Option<String> {
        Some(self.to_string())
    }
}

#[derive(Debug)]
pub struct InternalServerError(String); // just for have a concrete error type

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request Validation Error ({:?})", self.0)
    }
}

impl<E: Error> From<E> for InternalServerError {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}

impl ErrorKind for InternalServerError {
    fn code(&self) -> u16 {
        0
    }

    fn message(&self) -> String {
        r"Recommender Request Error".to_owned()
    }

    fn report(&self) -> Option<String> {
        Some(self.0.to_owned())
    }

    fn from_none() -> Option<Self> {
        Some(Self("None Value".to_owned()))
    }
}
