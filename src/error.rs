use actix_web::{http::StatusCode, HttpResponse};

use crate::models::response::ResponseBody;

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: impl ServiceErrorMessage) -> Self {
        ServiceError {
            http_status,
            body: ResponseBody {
                data: message.text(),
                success: false,
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

pub trait ServiceErrorMessage {
    fn text(&self) -> String;
}

#[macro_export]
macro_rules! messages_enum {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
            $($variant),*
        }

        impl ServiceErrorMessage for $name {
            fn text(&self) -> String {
                match self {
                    $($name::$variant => String::from(stringify!($variant))),*
                }
            }
        }
    };
}
