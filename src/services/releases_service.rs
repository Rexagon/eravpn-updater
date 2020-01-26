use {chrono::Utc, diesel::prelude::*};

use crate::{
    config::db::Pool, messages_enum, models::release::Release, models::release::ReleaseCreationData,
};

pub fn all_releases(pool: &Pool) -> Vec<Release> {
    use crate::schema::releases;

    let connection = &pool.get().unwrap();

    let result = releases::table.load::<Release>(connection).unwrap();

    result
}

pub fn get_release(version: (i32, i32, i32), pool: &Pool) -> Result<Release, ReleasesServiceError> {
    use crate::schema::releases;

    let connection = &pool.get().unwrap();

    let release: Release = releases::table
        .filter(releases::columns::version_major.eq(version.0))
        .filter(releases::columns::version_minor.eq(version.1))
        .filter(releases::columns::version_patch.eq(version.2))
        .get_result(connection)
        .map_err(|err| {
            error!("{:?}", err);
            ReleasesServiceError::ReleaseNotFound
        })?;

    Ok(release)
}

pub fn create_release(version: (i32, i32, i32), pool: &Pool) -> Result<i32, ReleasesServiceError> {
    use crate::schema::releases;

    let connection = &pool.get().unwrap();

    let release = ReleaseCreationData {
        version_major: version.0,
        version_minor: version.1,
        version_patch: version.2,
        creation_date: Utc::now().naive_utc(),
    };

    let id = diesel::insert_into(releases::table)
        .values(release)
        .returning(releases::columns::id)
        .get_result(connection)
        .map_err(|err| {
            error!("{:?}", err);

            ReleasesServiceError::ReleaseCreationError
        })?;

    Ok(id)
}

messages_enum! {
    pub enum ReleasesServiceError {
        ReleaseCreationError,
        ReleaseNotFound,
    }
}
