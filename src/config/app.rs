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
                        web::resource("/signin")
                            .route(web::post().to(account_controller::sign_in))
                    )
                    .service(
                        web::resource("/signup")
                            .route(web::post().to(account_controller::sign_up))
                    )
            )
            .service(
                web::resource("/releases")
                    .route(web::get().to(releases_controller::all_releases))
            )
    );
}
