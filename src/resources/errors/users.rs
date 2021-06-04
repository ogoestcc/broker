use std::{error::Error, fmt::Display};

use crate::resources::errors::ErrorKind;

use super::{InternalServerError, ServiceError};

#[derive(Debug)]
pub enum UsersError {
    NotFound,
    Inactive,
    Internal(InternalServerError),
}

impl UsersError {
    pub fn not_found() -> Self {
        Self::NotFound
    }
}

impl ErrorKind for UsersError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 10,
            Self::Inactive => 11,
            Self::Internal(internal) => internal.code(),
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Internal(internal) => internal.message(),
            _ => self.to_string(),
        }
    }

    fn report(&self) -> Option<String> {
        match self {
            Self::Internal(internal) => internal.report(),
            _ => None,
        }
    }
}

impl From<InternalServerError> for UsersError {
    fn from(err: InternalServerError) -> Self {
        Self::Internal(err)
    }
}

impl Display for UsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "User Not Found"),
            Self::Inactive => write!(f, "User Inactive"),
            Self::Internal(err) => write!(f, "Internal Error ({:?})", err),
        }
    }
}

impl<E: Error> From<E> for UsersError {
    fn from(err: E) -> Self {
        Self::Internal(InternalServerError::from(err))
    }
}

impl From<UsersError> for ServiceError<UsersError> {
    fn from(err: UsersError) -> Self {
        match err {
            UsersError::Internal(_) => Self::internal(err),
            _ => Self::not_found(err),
        }
    }
}
