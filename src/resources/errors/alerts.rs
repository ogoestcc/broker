use std::{error::Error, fmt::Display};

use crate::resources::errors::ErrorKind;

use super::{InternalServerError, ServiceError};

#[derive(Debug, serde::Serialize)]
pub enum AlertsError {
    Internal(InternalServerError),
}

impl ErrorKind for AlertsError {
    fn code(&self) -> u16 {
        match self {
            Self::Internal(internal) => internal.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Internal(internal) => internal.message(),
            _ => self.to_string(),
        }
    }

    type Report = InternalServerError;
    fn report(&self) -> Option<Self::Report> {
        match self {
            Self::Internal(internal) => internal.report(),
            _ => None,
        }
    }
}

impl From<InternalServerError> for AlertsError {
    fn from(err: InternalServerError) -> Self {
        Self::Internal(err)
    }
}

impl Display for AlertsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(err) => write!(f, "Internal Error ({:?})", err),
        }
    }
}

impl<E: Error> From<E> for AlertsError {
    fn from(err: E) -> Self {
        Self::Internal(InternalServerError::from(err))
    }
}

impl From<AlertsError> for ServiceError<AlertsError> {
    fn from(err: AlertsError) -> Self {
        match err {
            AlertsError::Internal(_) => Self::internal(err),
            _ => Self::not_found(err),
        }
    }
}
