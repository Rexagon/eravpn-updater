use actix_web::{http::StatusCode, web, Responder};

use crate::response::ApiResponse;
use crate::{
    config::db::Pool,
    models::account::{SignInData, SignUpData},
    response::ServiceError,
    services::{account_service, account_token_service::TOKEN_TYPE},
};

#[derive(Serialize)]
pub struct TokenResponseBody {
    pub token: String,
    pub token_type: String,
}

pub async fn sign_in(data: web::Json<SignInData>, pool: web::Data<Pool>) -> impl Responder {
    match account_service::sign_in(data.0, &pool) {
        Ok(token) => Ok(ApiResponse::new(TokenResponseBody {
            token,
            token_type: TOKEN_TYPE.to_string(),
        })),
        Err(err) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, err)),
    }
}

pub async fn sign_up(data: web::Json<SignUpData>, pool: web::Data<Pool>) -> impl Responder {
    match account_service::sign_up(data.0, &pool) {
        Ok(_) => Ok(ApiResponse::empty()),
        Err(err) => Err(ServiceError::new(StatusCode::BAD_REQUEST, err)),
    }
}
