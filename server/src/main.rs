use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use std::path::Path;

#[get("/")]
async fn serve_root() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .encoding(ContentEncoding::Auto)
        .body(include_str!("../../client/dist/index.html"))
}
#[get("/{file}")]
async fn serve_static_files(web::Path(file): web::Path<String>) -> HttpResponse {
    let bytes = macros::match_raw_files!(["../..", "file", "client/dist", "client/public"]);
    HttpResponse::Ok()
        .content_type(
            mime_guess::from_path(Path::new(&file))
                .first()
                .unwrap()
                .essence_str(),
        )
        .encoding(ContentEncoding::Auto)
        .body(bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .service(serve_root)
            .service(serve_static_files)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
