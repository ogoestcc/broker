use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error, HttpMessage,
};
use futures::future::{self, Either, Ready};

use crate::{
    config::Config,
    resources::errors::{auth::AuthError, ServiceError},
    utils::jwt,
};

pub struct Auth;
pub struct AuthMiddleware<S>(S);

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthMiddleware(service))
    }
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        let config = req.app_data::<web::Data<Config>>().unwrap();
        let secret = config.auth.secret_key.as_bytes();

        match token {
            Some(token) => match jwt::decode(token.to_str().unwrap().into(), secret) {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    Either::Left(self.0.call(req))
                }
                Err(_) => Either::Right(future::err(
                    ServiceError::new(AuthError::InvalidToken).into(),
                )),
            },
            None => Either::Right(future::err(
                ServiceError::new(AuthError::TokenNotFound).into(),
            )),
        }
    }
}
