use jsonwebtoken::Header;

use crate::models::account_token::AccountToken;

pub static TOKEN_TYPE: &'static str = "bearer";

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
