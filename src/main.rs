#[macro_use]
extern crate log;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod api;
mod config;
mod models;
mod schema;

use {
    actix_web::{HttpServer, App},
    actix_service::Service,
    futures::FutureExt,
    std::{io, env},
};
use actix_web::dev::Factory;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not specified");
    let app_post = env::var("APP_PORT").expect("APP_PORT not specified");
    let app_url = format!("{}:{}", &app_host, &app_post);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not specified");

    let pool = config::db::migrate_and_configure(&db_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::configure_services)
    })
        .bind(&app_url)?
        .run()
        .await
}
