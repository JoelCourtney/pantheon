use actix_web::{post, Responder, HttpResponse};

#[post("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
