use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Release {
    pub id: i32,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub creation_date: NaiveDateTime,
    pub active: bool,
    pub description: Option<String>,
    pub changelog: Option<String>,
}
