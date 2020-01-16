use {bcrypt, diesel::prelude::*};

use crate::{
    config::db::Pool,
    messages_enum,
    models::account::{Account, LoginDto},
    schema::accounts::dsl::*,
    services::account_token_service,
};

#[derive(Serialize)]
pub struct TokenResponseBody {
    pub token: String,
    pub token_type: String,
}

pub fn login(login: LoginDto, pool: &Pool) -> Result<TokenResponseBody, AccountServiceError> {
    let connection = &pool.get().unwrap();

    let account_to_verify: Account = match accounts
        .filter(username.eq(&login.username))
        .get_result(connection)
    {
        Ok(acc) => acc,
        Err(_) => {
            info!("Not found");
            return Err(AccountServiceError::WrongCredentials);
        }
    };

    info!("Found account: {:?}", account_to_verify);

    match bcrypt::verify(&login.password, &account_to_verify.password) {
        Ok(true) => {
            return Ok(TokenResponseBody {
                token: account_token_service::generate(login.username),
                token_type: account_token_service::TOKEN_TYPE.to_string(),
            });
        }
        Err(err) => info!("{:?}", err),
        _ => (),
    }

    return Err(AccountServiceError::WrongCredentials);
}

messages_enum! {
    pub enum AccountServiceError {
        WrongCredentials,
    }
}
