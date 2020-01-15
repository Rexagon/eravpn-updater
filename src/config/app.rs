use actix_web::web;

use crate::api::*;

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring services");

    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
    );
}
