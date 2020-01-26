use actix_web::{web, Responder};

use crate::{
    config::db::Pool,
    models::release::ReleaseResponse,
    response::{ApiResponse, ServiceError},
    services::releases_service::{self, ReleasesServiceError},
};

pub async fn all_releases(pool: web::Data<Pool>) -> impl Responder {
    ApiResponse::new(
        releases_service::all_releases(&pool)
            .iter_mut()
            .map(ReleaseResponse::new)
            .collect::<Vec<ReleaseResponse>>(),
    )
}

pub async fn get_release(
    version: web::Path<(u16, u16, u16)>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let version = (version.0.into(), version.1.into(), version.2.into());

    releases_service::get_release(version, &pool)
        .map(|mut release| ReleaseResponse::new(&mut release))
        .map(ApiResponse::new)
        .map_err(|err| match err {
            ReleasesServiceError::ReleaseNotFound => ServiceError::not_found(err),
            _ => ServiceError::bad_request(err),
        })
}

pub async fn create_release(
    version: web::Path<(u16, u16, u16)>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let version = (version.0.into(), version.1.into(), version.2.into());

    match releases_service::create_release(version, &pool) {
        Ok(id) => Ok(ApiResponse::new(id)),
        Err(err) => Err(ServiceError::bad_request(err)),
    }
}
