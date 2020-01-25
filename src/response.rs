use serde::export::Formatter;
use {
    actix_web::{error, http::StatusCode, Error, HttpRequest, HttpResponse, Responder},
    futures::future::{ready, Ready},
    serde::Serialize,
};

#[derive(Debug, Serialize)]
pub enum ApiResponse<T> {
    Data(T),
    Empty,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        ApiResponse::Data(data)
    }
}

impl ApiResponse<()> {
    pub fn empty() -> Self {
        ApiResponse::Empty
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(match self {
            ApiResponse::Data(data) => HttpResponse::Ok().json(data),
            ApiResponse::Empty => HttpResponse::new(StatusCode::default()),
        }))
    }
}

#[derive(Debug)]
pub struct ServiceError {
    http_status: StatusCode,
    body: ErrorResponseBody,
}

#[derive(Debug, Serialize)]
struct ErrorResponseBody {
    error_code: String,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: impl ToString) -> Self {
        ServiceError {
            http_status,
            body: ErrorResponseBody {
                error_code: message.to_string(),
            },
        }
    }
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        self.http_status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(&self.body)
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.body.error_code)
    }
}

#[macro_export]
macro_rules! messages_enum {
    ($visibility:vis enum $name:ident {
        $($variant:ident),*,
    }) => {
        $visibility enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $($name::$variant => String::from(stringify!($variant))),*
                }
            }
        }
    };
}
