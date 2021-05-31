use actix_web::web::Json;
use actix_web::FromRequest;
use futures::future::Future;
use std::pin::Pin;
use validator::Validate;

use crate::resources::errors::{auth::AuthError, ServiceError};

pub struct Validator<O>(O)
where
    O: Validate;

impl<O: Validate> Validator<O> {
    pub fn into_inner(self) -> O {
        self.0
    }
}

impl<O: 'static> FromRequest for Validator<O>
where
    O: Validate + for<'de> serde::Deserialize<'de>,
{
    type Error = ServiceError<AuthError>;

    type Future = Pin<Box<dyn Future<Output = Result<Validator<O>, Self::Error>>>>;

    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let json = <Json<O>>::from_request(req, payload);
        Box::pin(async {
            let json = json.await.unwrap();

            json.validate()
                .map(|_| Ok(Validator(json.0)))
                .map_err(|err| ServiceError::new(AuthError::InvalidRequest(err.to_string())))?
        })
    }
}
