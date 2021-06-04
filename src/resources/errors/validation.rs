use std::{error::Error, fmt::Display};

#[derive(Debug)]
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

    fn report(&self) -> Option<String> {
        Some(self.0.to_owned())
    }
}