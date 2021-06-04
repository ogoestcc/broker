use std::{error::Error, fmt::Display};

use super::ServiceError;

#[derive(Debug, serde::Serialize)]
pub struct ValidationError(String); // just for have a concrete error type for request validations

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request Validation Error ({:?})", self.0)
    }
}

impl<E: Error> From<E> for ValidationError {
    fn from(err: E) -> Self {
        Self(err.to_string())
    }
}

impl super::ErrorKind for ValidationError {
    fn code(&self) -> u16 {
        0
    }

    fn message(&self) -> String {
        r"Invalid Request".to_owned()
    }

    type Report = String;
    fn report(&self) -> Option<String> {
        Some(self.0.to_owned())
    }
}

impl From<ValidationError> for ServiceError<ValidationError> {
    fn from(err: ValidationError) -> Self {
        ServiceError::bad_request(err)
    }
}
