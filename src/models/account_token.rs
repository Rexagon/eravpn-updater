use jsonwebtoken::Header;

pub static TYPE: &'static str = "bearer";

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct AccountToken {
    pub iat: i64,
    pub exp: i64,
    pub username: String,
}

impl AccountToken {
    pub fn generate(username: String) -> String {
        let now = chrono::offset::Utc::now().timestamp();
        let payload = AccountToken {
            iat: now,
            exp: now + ONE_WEEK,
            username,
        };

        jsonwebtoken::encode(&Header::default(), &payload, &KEY).unwrap()
    }
}
