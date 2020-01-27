use actix_web::web;

use crate::{
    api_response::{ApiResponse, ResponseBody, ServiceError},
    config::db::Pool,
    models::release::ReleaseResponse,
    services::releases_service::{self, ReleasesServiceError},
};

pub async fn all_releases(pool: web::Data<Pool>) -> ResponseBody<Vec<ReleaseResponse>> {
    releases_service::all_releases(&pool)
        .iter_mut()
        .map(ReleaseResponse::new)
        .collect::<Vec<ReleaseResponse>>()
        .into()
}

pub async fn get_release(
    version: web::Path<(u16, u16, u16)>,
    pool: web::Data<Pool>,
) -> ApiResponse<ReleaseResponse> {
    let version = (version.0.into(), version.1.into(), version.2.into());

    let release = releases_service::get_release(version, &pool)
        .map(|mut release| ReleaseResponse::new(&mut release))
        .map_err(|err| match err {
            ReleasesServiceError::ReleaseNotFound => ServiceError::not_found(err),
            _ => ServiceError::bad_request(err),
        })?;

    Ok(release.into())
}

pub async fn create_release(
    version: web::Path<(u16, u16, u16)>,
    pool: web::Data<Pool>,
) -> ApiResponse<i32> {
    let version = (version.0.into(), version.1.into(), version.2.into());

    let id = releases_service::create_release(version, &pool).map_err(ServiceError::bad_request)?;

    Ok(id.into())
}
