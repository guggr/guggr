use std::{
    future::{Ready, ready},
    sync::Arc,
};

use actix_web::{
    Error, HttpMessage, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error,
    http::header,
    web,
};
use futures_util::future::{self, LocalBoxFuture};
use tracing::error;

use crate::core::{models::auth::UserId, ports::service::ServicePort};

/// Returns a 401 response with `www-authenticate` header containing the value
/// `Bearer`
fn unauthorized_with_bearer() -> actix_web::Error {
    let resp = HttpResponse::Unauthorized()
        .insert_header((header::WWW_AUTHENTICATE, "Bearer"))
        .finish();

    error::InternalError::from_response("", resp).into()
}

/// Is the `AuthMiddleware` factory and builds it
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

/// Authentication middleware which checks the supplied JWT access token.
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
        let svc = if let Some(s) = req.app_data::<web::Data<Arc<dyn ServicePort>>>() {
            s.get_ref()
        } else {
            error!("Service is not configured");
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

        let verified_user_id = match svc.validate_access_token(token) {
            Ok(u) => u,
            Err(_) => return Box::pin(futures_util::future::err(unauthorized_with_bearer())),
        };

        // Pass the user ID to the endpoint handlers.
        let u = UserId(verified_user_id);
        req.extensions_mut().insert(u);

        let fut = self.service.call(req);
        Box::pin(fut)
    }
}
