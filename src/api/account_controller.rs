use actix_web::{http::StatusCode, web, HttpResponse};

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::account::{SignInData, SignUpData},
    models::response::ResponseBody,
    services::{account_service, account_token_service::TOKEN_TYPE},
};

#[derive(Serialize)]
pub struct TokenResponseBody {
    pub token: String,
    pub token_type: String,
}

pub async fn sign_in(data: web::Json<SignInData>, pool: web::Data<Pool>) -> HttpResponse {
    match account_service::sign_in(data.0, &pool) {
        Ok(token) => HttpResponse::Ok().json(ResponseBody::new(
            TokenResponseBody {
                token,
                token_type: TOKEN_TYPE.to_string(),
            },
            true,
        )),
        Err(err) => ServiceError::new(StatusCode::UNAUTHORIZED, err).response(),
    }
}

pub async fn sign_up(data: web::Json<SignUpData>, pool: web::Data<Pool>) -> HttpResponse {
    match account_service::sign_up(data.0, &pool) {
        Ok(_) => HttpResponse::Ok().json(ResponseBody::new((), true)),
        Err(err) => ServiceError::new(StatusCode::BAD_REQUEST, err).response(),
    }
}
