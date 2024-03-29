use actix_web::{get, HttpResponse, Responder };

#[get("/scheduler/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("test")
}
