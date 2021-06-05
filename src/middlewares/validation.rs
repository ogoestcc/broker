use actix_web::web::{Json, Query};
use actix_web::FromRequest;
use futures::future::LocalBoxFuture;
use std::ops::Deref;
use validator::Validate;

use crate::resources::errors::validation::ValidationError;
use crate::resources::errors::ServiceError;

pub trait Extractor {
    type Value: Validate;
    fn take(self) -> Self::Value;
}

impl<V> Extractor for Json<V>
where
    V: Validate,
{
    type Value = V;

    fn take(self) -> Self::Value {
        self.into_inner()
    }
}

impl<V> Extractor for Query<V>
where
    V: Validate,
{
    type Value = V;

    fn take(self) -> Self::Value {
        self.into_inner()
    }
}

pub struct Validator<T>(T::Value)
where
    T: Extractor,
    T::Value: 'static + Validate;

impl<E> FromRequest for Validator<E>
where
    E: Extractor + FromRequest + Deref,
    E::Value: Validate + for<'de> serde::Deserialize<'de> + 'static,
    E::Error: std::error::Error,
    E::Future: 'static,
{
    type Error = ServiceError<ValidationError>;

    type Future = LocalBoxFuture<'static, Result<Validator<E>, Self::Error>>;

    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extractor = E::from_request(req, payload);

        Box::pin(async {
            let value = extractor.await.map_err(ValidationError::from)?.take();
            value.validate().map_err(ValidationError::from)?;
            Ok(Validator(value))
        })
    }
}

impl<E: Extractor> Validator<E> {
    pub fn into_inner(self) -> E::Value {
        self.0
    }
}

impl<E> std::fmt::Debug for Validator<E>
where
    E: Extractor,
    E::Value: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<E> Deref for Validator<E>
where
    E: Extractor,
{
    type Target = E::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
