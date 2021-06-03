use actix_web::{
    http::{header, StatusCode},
    ResponseError,
};

pub mod auth;
pub mod users;

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

pub trait ErrorKind {
    fn code(&self) -> u16;
    fn message(&self) -> String;
    fn status_code(&self) -> StatusCode;
    fn report(&self) -> Option<String>;
}
#[derive(Debug, Clone)]
pub struct ServiceError<K: ErrorKind>(K, Option<String>);

impl<K> ServiceError<K>
where
    K: ErrorKind,
{
    pub fn new(kind: K) -> Self {
        Self(kind, None)
    }
}

impl<K: ErrorKind> std::fmt::Display for ServiceError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = ErrorMessage {
            code: self.0.code(),
            message: self.0.message(),
            report: self.0.report(),
        };

        write!(
            f,
            "{}",
            serde_json::to_string(&ErrorBody { error }).unwrap()
        )
    }
}

impl<K: ErrorKind + std::fmt::Debug> ResponseError for ServiceError<K> {
    fn status_code(&self) -> StatusCode {
        self.0.status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut resp = actix_web::HttpResponse::new(self.status_code());
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );
        resp.set_body(actix_web::dev::Body::from(self.to_string()))
    }
}

#[derive(Debug)]
pub struct InternalServerError(pub Option<String>);

impl ErrorKind for InternalServerError {
    fn code(&self) -> u16 {
        0
    }

    fn message(&self) -> String {
        "Internal Server Error".to_owned()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn report(&self) -> Option<String> {
        self.0.clone()
    }
}
