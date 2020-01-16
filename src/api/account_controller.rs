use actix_web::{web, HttpResponse};

use crate::{
    config::db::Pool, models::account::LoginDto, models::response::ResponseBody,
    services::account_service,
};

pub async fn login(login_dto: web::Json<LoginDto>, pool: web::Data<Pool>) -> HttpResponse {
    match account_service::login(login_dto.0, &pool) {
        Ok(res) => HttpResponse::Ok().json(ResponseBody::new(res, true)),
        Err(err) => err.response(),
    }
}
