use actix_web::{http::StatusCode, web, HttpResponse};

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::account::LoginDto,
    models::response::ResponseBody,
    services::{account_service, account_token_service::TOKEN_TYPE},
};

#[derive(Serialize)]
pub struct TokenResponseBody {
    pub token: String,
    pub token_type: String,
}

pub async fn login(login_dto: web::Json<LoginDto>, pool: web::Data<Pool>) -> HttpResponse {
    match account_service::login(login_dto.0, &pool) {
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
