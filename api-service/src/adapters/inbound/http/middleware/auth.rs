use std::future::{Ready, ready};

use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error,
    http::header,
    web,
};
use compact_jwt::JwsEs256Signer;
use futures_util::future::{self, LocalBoxFuture};
use tracing::error;

use crate::core::domain::auth_helper::verify_jwt;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let signer = match req.app_data::<web::Data<JwsEs256Signer>>() {
            Some(s) => s.get_ref(),
            None => {
                error!("JWT signer is not configured");
                return Box::pin(future::err::<ServiceResponse<B>, _>(
                    error::ErrorInternalServerError("JWT signer not configured"),
                ));
            }
        };

        let token = match req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
        {
            Some(t) if !t.is_empty() => t,
            _ => {
                return Box::pin(future::err::<ServiceResponse<B>, _>(
                    error::ErrorUnauthorized(""),
                ));
            }
        };

        if verify_jwt(signer, token).is_err() {
            return Box::pin(future::err::<ServiceResponse<B>, _>(
                error::ErrorUnauthorized(""),
            ));
        }
        let fut = self.service.call(req);
        Box::pin(fut)
    }
}
