use actix_web::{post, Responder, HttpResponse};
use crate::filesystem;

#[post("/list")]
pub async fn list() -> impl Responder {
    let list_result = filesystem::get_lists().await;
    match list_result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(error) => HttpResponse::from_error(error)
    }
}
