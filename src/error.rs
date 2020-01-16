use actix_web::{http::StatusCode, HttpResponse};

use crate::models::response::ResponseBody;

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: impl ToString) -> Self {
        ServiceError {
            http_status,
            body: ResponseBody {
                data: message.to_string(),
                success: false,
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

#[macro_export]
macro_rules! messages_enum {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
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
