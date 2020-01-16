use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    error::ServiceError,
    messages_enum,
    models::account::{Account, LoginDto},
    models::account_token::{self, AccountToken},
};

#[derive(Serialize)]
pub struct TokenResponseBody {
    pub token: String,
    pub token_type: String,
}

pub fn login(login: LoginDto, pool: &web::Data<Pool>) -> Result<TokenResponseBody, ServiceError> {
    match Account::login(login, &pool.get().unwrap()) {
        Some(login_info) => Ok(TokenResponseBody {
            token: AccountToken::generate(login_info.username),
            token_type: account_token::TYPE.to_string(),
        }),
        None => Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            AccountServiceError::WrongCredentials,
        )),
    }
}

messages_enum! {
    enum AccountServiceError {
        WrongCredentials,
    }
}
