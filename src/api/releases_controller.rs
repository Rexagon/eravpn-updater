use actix_web::{web, Responder};

use crate::{
    config::db::Pool, models::release::Release, response::ApiResponse, services::releases_service,
};

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
    fn new(data: &mut Release) -> Self {
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

pub async fn all_releases(pool: web::Data<Pool>) -> impl Responder {
    ApiResponse::new(
        releases_service::all_releases(&pool)
            .iter_mut()
            .map(ReleaseResponse::new)
            .collect::<Vec<ReleaseResponse>>(),
    )
}
