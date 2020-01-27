use {
    diesel::prelude::*,
    jsonwebtoken::{Header, TokenData, Validation},
};

use crate::models::account::Account;
use crate::{config::db::Pool, models::account_token::AccountToken};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

pub fn generate(username: String) -> String {
    let now = chrono::offset::Utc::now().timestamp();
    let payload = AccountToken {
        iat: now,
        exp: now + ONE_WEEK,
        username,
    };

    jsonwebtoken::encode(&Header::default(), &payload, &KEY).unwrap()
}

pub fn decode(token: String) -> jsonwebtoken::errors::Result<TokenData<AccountToken>> {
    jsonwebtoken::decode::<AccountToken>(&token, &KEY, &Validation::default())
}

pub fn verify(data: TokenData<AccountToken>, pool: &Pool) -> Option<AccountToken> {
    use crate::schema::accounts;

    let connection = &pool.get().unwrap();

    if accounts::table
        .filter(accounts::columns::username.eq(&data.claims.username))
        .get_result::<Account>(connection)
        .is_ok()
    {
        Some(data.claims)
    } else {
        None
    }
}
