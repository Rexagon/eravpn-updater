use {bcrypt, diesel::prelude::*};

use crate::{
    config::db::Pool,
    messages_enum,
    models::account::{Account, LoginDto},
    schema::accounts::dsl::*,
    services::account_token_service,
};

pub fn login(login: LoginDto, pool: &Pool) -> Result<String, AccountServiceError> {
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
            return Ok(account_token_service::generate(login.username));
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
