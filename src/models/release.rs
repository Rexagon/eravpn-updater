use chrono::NaiveDateTime;

use crate::schema::releases;

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

#[derive(Debug, Insertable)]
#[table_name = "releases"]
pub struct ReleaseCreationData {
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub creation_date: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ReleaseResponse {
    pub id: i32,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub creation_date: i64,
    pub active: bool,
    pub description: Option<String>,
    pub changelog: Option<String>,
}

impl ReleaseResponse {
    pub fn new(data: &mut Release) -> Self {
        ReleaseResponse {
            id: data.id,
            version_major: data.version_major,
            version_minor: data.version_minor,
            version_patch: data.version_patch,
            creation_date: data.creation_date.timestamp() * 1000,
            active: data.active,
            description: data.description.take(),
            changelog: data.changelog.take(),
        }
    }
}
