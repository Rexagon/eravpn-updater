use {
    actix_service::{Service, Transform},
    actix_web::{
        dev::{ServiceRequest, ServiceResponse},
        http::StatusCode,
        Error,
    },
    futures::{
        future::{ok, Ready},
        Future,
    },
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

use crate::{
    api_response::ServiceError, config::db::Pool, constants, services::account_token_service,
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        if self.check_auth(&req) {
            let future = self.service.call(req);

            Box::pin(async move {
                let res = future.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Err(ServiceError::new(StatusCode::UNAUTHORIZED, "Unauthorized").into())
            })
        }
    }
}

impl<S> AuthenticationMiddleware<S> {
    fn check_auth(&mut self, req: &ServiceRequest) -> bool {
        for ignored_route in constants::IGNORE_AUTH.iter() {
            if req.path().starts_with(ignored_route) {
                return true;
            }
        }

        let header = match req.headers().get(constants::AUTHORIZATION_HEADER) {
            Some(header) => header,
            None => return false,
        };

        let header = match header.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        if !header.starts_with("Bearer") {
            return false;
        }

        let token = header[6..header.len()].trim();
        let token_data = match account_token_service::decode(token.to_string()) {
            Ok(data) => data,
            Err(_) => return false,
        };

        let pool = req.app_data::<Pool>().unwrap();

        account_token_service::verify(token_data, &pool).is_some()
    }
}
