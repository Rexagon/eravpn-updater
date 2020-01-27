#[derive(Serialize, Deserialize)]
pub struct AccountToken {
    pub iat: i64,
    pub exp: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct AccountTokenResponse {
    pub token: String,
    pub token_type: String,
}

impl AccountTokenResponse {
    pub fn new(token: String) -> Self {
        AccountTokenResponse {
            token,
            token_type: TOKEN_TYPE.to_string(),
        }
    }
}

pub static TOKEN_TYPE: &'static str = "Bearer";
