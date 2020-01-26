use diesel::prelude::*;

use crate::{config::db::Pool, models::release::Release};

pub fn all_releases(pool: &Pool) -> Vec<Release> {
    use crate::schema::releases;

    let connection = &pool.get().unwrap();

    let result = releases::table.load::<Release>(connection).unwrap();

    result
}
