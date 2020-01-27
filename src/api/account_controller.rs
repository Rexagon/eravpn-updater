use actix_web::web;

use crate::{
    api_response::{ApiResponse, ServiceError},
    config::db::Pool,
    models::{
        account::{SignInData, SignUpData},
        account_token::AccountTokenResponse,
    },
    services::account_service,
};

pub async fn sign_in(
    data: web::Json<SignInData>,
    pool: web::Data<Pool>,
) -> ApiResponse<AccountTokenResponse> {
    let token =
        account_service::sign_in(data.0, &pool).map_err(|_| ServiceError::unauthorized())?;

    Ok(AccountTokenResponse::new(token).into())
}

pub async fn sign_up(data: web::Json<SignUpData>, pool: web::Data<Pool>) -> ApiResponse<()> {
    account_service::sign_up(data.0, &pool).map_err(ServiceError::bad_request)?;

    Ok(().into())
}
