use crate::schema::accounts;

#[derive(Debug, Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "accounts"]
pub struct SignUpData {
    pub username: String,
    pub password: String,
}
