use actix_web::web;

use crate::api::*;

#[rustfmt::skip]
pub fn configure_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring services");

    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/login")
                            .route(web::post().to(account_controller::login))
                    )
            )
    );
}
