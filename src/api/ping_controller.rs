use actix_web::HttpResponse;

#[get("/ping")]
pub fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("pong")
}
