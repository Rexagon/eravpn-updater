use diesel::prelude::*;

use crate::{
    config::db::{Connection, Pool},
    messages_enum,
    models::account::{Account, SignInData, SignUpData},
    services::account_token_service,
};

pub fn sign_in(data: SignInData, pool: &Pool) -> Result<String, AccountServiceError> {
    use crate::schema::accounts;

    let connection = &pool.get().unwrap();

    let account_to_verify: Account = match accounts::table
        .filter(accounts::columns::username.eq(&data.username))
        .get_result(connection)
    {
        Ok(acc) => acc,
        Err(_) => {
            info!("Not found");
            return Err(AccountServiceError::WrongCredentials);
        }
    };

    info!("Found account: {:?}", account_to_verify);

    match bcrypt::verify(&data.password, &account_to_verify.password) {
        Ok(true) => {
            return Ok(account_token_service::generate(data.username));
        }
        Err(err) => info!("{:?}", err),
        _ => (),
    }

    return Err(AccountServiceError::WrongCredentials);
}

pub fn sign_up(data: SignUpData, pool: &Pool) -> Result<(), AccountServiceError> {
    use crate::schema::accounts;

    let connection = &pool.get().unwrap();

    if find_account_by_username(&data.username, connection).is_ok() {
        return Err(AccountServiceError::AccountAlreadyExists);
    }

    let hashed_password = bcrypt::hash(&data.password, 6).unwrap();
    let account = SignUpData {
        username: data.username,
        password: hashed_password,
    };

    diesel::insert_into(accounts::table)
        .values(&account)
        .execute(connection)
        .unwrap();

    Ok(())
}

fn find_account_by_username(username: &str, connection: &Connection) -> QueryResult<Account> {
    use crate::schema::accounts;

    accounts::table
        .filter(accounts::columns::username.eq(username))
        .get_result::<Account>(connection)
}

messages_enum! {
    pub enum AccountServiceError {
        WrongCredentials,
        AccountAlreadyExists,
    }
}
