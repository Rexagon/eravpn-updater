use {bcrypt, diesel::prelude::*};

use crate::{config::db::Connection, schema::accounts::dsl::*};

#[derive(Debug, Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginInfoDto {
    pub username: String,
}

impl Account {
    pub fn login(login: LoginDto, conn: &Connection) -> Option<LoginInfoDto> {
        let account_to_verify: Account = match accounts
            .filter(username.eq(&login.username))
            .get_result(conn)
        {
            Ok(acc) => acc,
            Err(_) => {
                info!("Not found");
                return None;
            }
        };

        info!("Found account: {:?}", account_to_verify);

        match bcrypt::verify(&login.password, &account_to_verify.password) {
            Ok(true) => {
                return Some(LoginInfoDto {
                    username: account_to_verify.username,
                });
            }
            Err(err) => info!("{:?}", err),
            _ => (),
        }

        None
    }
}
