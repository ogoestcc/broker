use actix_web::web::{Json, Query};
use actix_web::FromRequest;
use futures::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use validator::Validate;

use crate::resources::errors::{auth::AuthError, ServiceError};

#[derive(Clone)]
pub struct Validator<O, T>(O, Rc<T>)
where
    T: FromRequest,
    O: Validate;

impl<O, T> Validator<O, T>
where
    O: Validate + Clone,
    T: FromRequest,
{
    pub fn into_inner(self) -> O {
        self.0
    }
}

impl<O, T> std::fmt::Debug for Validator<O, T>
where
    T: FromRequest,
    O: std::fmt::Debug + Validate,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<O: 'static + Clone> FromRequest for Validator<O, Json<O>>
where
    O: Validate + for<'de> serde::Deserialize<'de>,
{
    type Error = ServiceError<AuthError>;

    type Future = Pin<Box<dyn Future<Output = Result<Validator<O, Json<O>>, Self::Error>>>>;

    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let json = <Json<O>>::from_request(req, payload);
        Box::pin(async {
            let json =
                Rc::new(json.await.map_err(|err| {
                    ServiceError::new(AuthError::InvalidRequest(err.to_string()))
                })?);

            json.validate()
                .map(|_| Ok(Validator((*json).0.clone(), json.clone())))
                .map_err(|err| ServiceError::new(AuthError::InvalidRequest(err.to_string())))?
        })
    }
}

impl<O: 'static + Clone> FromRequest for Validator<O, Query<O>>
where
    O: Validate + for<'de> serde::Deserialize<'de>,
{
    type Error = ServiceError<AuthError>;

    type Future = Pin<Box<dyn Future<Output = Result<Validator<O, Query<O>>, Self::Error>>>>;

    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let query = <Query<O>>::from_request(req, payload);
        Box::pin(async {
            let query =
                Rc::new(query.await.map_err(|err| {
                    ServiceError::new(AuthError::InvalidRequest(err.to_string()))
                })?);

            query
                .validate()
                .map(|_| Ok(Validator((*query).0.clone(), query.clone())))
                .map_err(|err| ServiceError::new(AuthError::InvalidRequest(err.to_string())))?
        })
    }
}
