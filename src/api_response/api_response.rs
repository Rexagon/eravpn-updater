use {
    actix_web::{Error, HttpRequest, HttpResponse, Responder},
    futures::future::{ready, Ready},
    serde::Serialize,
};

use crate::api_response::ServiceError;

pub type ApiResponse<T> = Result<ResponseBody<T>, ServiceError>;

#[derive(Debug, Serialize)]
pub struct ResponseBody<T>(T);

impl<T> Responder for ResponseBody<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok().json(self.0)))
    }
}

impl<T> From<T> for ResponseBody<T>
where
    T: Serialize,
{
    fn from(data: T) -> Self {
        ResponseBody(data)
    }
}
