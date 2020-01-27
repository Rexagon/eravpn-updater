use {
    actix_web::{error::ResponseError, http::StatusCode, HttpResponse},
    serde::Serialize,
    std::fmt::Formatter,
};

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

    pub fn bad_request(message: impl ToString) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn not_found(message: impl ToString) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn unauthorized() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "Unauthorized")
    }
}

impl ResponseError for ServiceError {
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
