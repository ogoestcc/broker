use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::{error::Error, fmt::Display};

pub mod auth;
pub mod users;
pub mod validation;

#[derive(Debug, serde::Serialize)]
pub struct ErrorMessage<K: serde::Serialize> {
    pub code: u16,
    pub message: String,
    #[cfg_attr(not(debug_assertions), serde(skip))]
    pub report: Option<K>,
}

#[derive(Debug, serde::Serialize)]
struct ErrorBody<K: serde::Serialize> {
    error: ErrorMessage<K>,
}

pub trait ErrorKind: Sized {
    fn code(&self) -> u16 {
        Default::default()
    }

    fn message(&self) -> String {
        Default::default()
    }

    type Report;

    fn report(&self) -> Option<Self::Report> {
        Default::default()
    }

    fn from_none() -> Option<Self> {
        Default::default()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum ServiceError<K>
where
    K: ErrorKind,
{
    BadRequest(K),          // 400
    Unauthorized(K),        // 401
    Forbidden(K),           // 403
    NotFound(K),            // 404
    PreconditionFailed(K),  // 412
    InternalServerError(K), // 500
}

impl<K, R> ServiceError<K>
where
    K: serde::Serialize + ErrorKind<Report = R>,
    R: serde::Serialize,
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

    fn get_error_body(&self) -> ErrorBody<R> {
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

impl<K, R> std::fmt::Display for ServiceError<K>
where
    R: serde::Serialize,
    K: ErrorKind<Report = R> + std::fmt::Debug + serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<K, R> ResponseError for ServiceError<K>
where
    R: serde::Serialize,
    K: ErrorKind<Report = R> + std::fmt::Debug + serde::Serialize,
{
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
            Self::BadRequest(_) => HttpResponse::BadRequest(),
            Self::Unauthorized(_) => HttpResponse::Unauthorized(),
            Self::Forbidden(_) => HttpResponse::Forbidden(),
            Self::NotFound(_) => HttpResponse::NotFound(),
            Self::PreconditionFailed(_) => HttpResponse::PreconditionFailed(),
            Self::InternalServerError(_) => HttpResponse::InternalServerError(),
        };

        builder.json(self.get_error_body())
    }
}

impl<K: std::error::Error + Clone> ErrorKind for K {
    type Report = K;
    fn report(&self) -> Option<Self::Report> {
        Some(self.to_owned())
    }
}

#[derive(Debug, serde::Serialize, Clone)]
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
    fn message(&self) -> String {
        r"Recommender Request Error".to_owned()
    }

    type Report = Self;
    fn report(&self) -> Option<Self::Report> {
        Some(self.to_owned())
    }

    fn from_none() -> Option<Self> {
        Some(Self("None Value".to_owned()))
    }
}

impl From<InternalServerError> for ServiceError<InternalServerError> {
    fn from(err: InternalServerError) -> Self {
        Self::internal(err)
    }
}
