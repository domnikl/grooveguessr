use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::auth::UserInfo;

pub struct AuthRequired;

impl<S, B> Transform<S, ServiceRequest> for AuthRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthRequiredMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRequiredMiddleware { service }))
    }
}
pub struct AuthRequiredMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthRequiredMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let session = request.request().get_session();

        let is_logged_in = match session.get::<UserInfo>("user_info") {
            Ok(x) => x.is_some(),
            Err(_) => false,
        };

        if !is_logged_in {
            let res = request.into_response(HttpResponse::Unauthorized().finish());

            Box::pin(async { Ok(ServiceResponse::map_into_right_body(res)) })
        } else {
            let fut = self.service.call(request);

            Box::pin(async move {
                let res = fut.await?;

                Ok(ServiceResponse::map_into_left_body(res))
            })
        }
    }
}
