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
