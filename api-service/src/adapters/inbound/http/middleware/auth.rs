use std::{
    future::{Ready, ready},
    sync::Arc,
};

use actix_web::{
    Error, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::{self},
    http::header,
    web,
};
use config::ApiServiceConfig;
use futures_util::future::{self, LocalBoxFuture};
use tracing::error;

use crate::core::{
    domain::auth_helper::{JwtSigner, get_unverified_user},
    ports::storage::StoragePort,
};

fn unauthorized_with_bearer() -> actix_web::Error {
    let resp = HttpResponse::Unauthorized()
        .insert_header((header::WWW_AUTHENTICATE, "Bearer"))
        .finish();

    error::InternalError::from_response("", resp).into()
}

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
        let storage = if let Some(s) = req.app_data::<web::Data<Arc<dyn StoragePort>>>() {
            s.get_ref()
        } else {
            error!("Storage is not configured");
            return Box::pin(future::err::<ServiceResponse<B>, _>(
                error::ErrorInternalServerError(""),
            ));
        };
        let config = if let Some(s) = req.app_data::<web::Data<ApiServiceConfig>>() {
            s.get_ref()
        } else {
            error!("Config is not configured");
            return Box::pin(future::err::<ServiceResponse<B>, _>(
                error::ErrorInternalServerError(""),
            ));
        };

        let token = match req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
        {
            Some(t) if !t.is_empty() => t,
            _ => return Box::pin(futures_util::future::err(unauthorized_with_bearer())),
        };
        let unverified_user = match get_unverified_user(token) {
            Ok(u) => u,
            Err(_) => return Box::pin(futures_util::future::err(unauthorized_with_bearer())),
        };

        let user = match storage.auth().get_user_jwt_secrets(&unverified_user) {
            Ok(u) => u,
            Err(_) => return Box::pin(futures_util::future::err(unauthorized_with_bearer())),
        };
        let signer = JwtSigner::new(
            &config.auth_secret(),
            user.jwt_salt.as_bytes(),
            user.jwt_secret.as_bytes(),
        );
        if signer.verify_access_token(token).is_err() {
            return Box::pin(futures_util::future::err(unauthorized_with_bearer()));
        }

        let fut = self.service.call(req);
        Box::pin(fut)
    }
}
