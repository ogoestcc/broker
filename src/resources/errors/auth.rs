use crate::resources::errors::ErrorKind;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
    UserNotFoundOrInvalidPassword(Option<String>),
}

impl AuthError {
    pub fn authentication_error<E: ToString>(error: E) -> Self {
        Self::UserNotFoundOrInvalidPassword(Some(error.to_string()))
    }
}

impl ErrorKind for AuthError {
    fn code(&self) -> u16 {
        match self {
            Self::InvalidToken => 1,
            Self::MissingToken => 2,
            Self::UserNotFoundOrInvalidPassword(_) => 3,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::InvalidToken => r"Invalid Token".to_owned(),
            Self::MissingToken => r"Missing Authentication Token".to_owned(),
            _ => r"Access Denied".to_owned(),
        }
    }

    fn report(&self) -> Option<String> {
        match self {
            Self::UserNotFoundOrInvalidPassword(report) => report.to_owned(),
            _ => None,
        }
    }
}
