mod filesystem;

use actix_web::http::header::ContentType;
use actix_web::http::ContentEncoding;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use std::path::{Path, PathBuf};
use colored::Colorize;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let prefix = match opt.dir {
        Some(path) => path,
        None => std::env::current_dir()?,
    };
    println!("Serving {} on {}{}", prefix.as_path().to_str().unwrap().green(), "http://localhost:".green(), opt.port.to_string().green());
    HttpServer::new(move || {
        App::new()
            .data(prefix.clone())
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .service(serve_root)
            .service(serve_static_files)
            .service(list_characters)
            .service(read_character)
            .service(write_character)
    })
    .bind(format!("127.0.0.1:{}", opt.port))?
    .run()
    .await
}

/// Server for DnDCent.
#[derive(StructOpt, Debug)]
#[structopt(name = "DnDCent")]
struct Opt {
    /// Path prefix to serve characters from (optional).
    #[structopt(short, long, parse(from_os_str))]
    dir: Option<PathBuf>,

    /// Port to host site on.
    #[structopt(short, long, default_value = "8080")]
    port: u32,
}

#[get("/")]
async fn serve_root() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
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
        .body(bytes)
}

#[post("/list")]
async fn list_characters(prefix: web::Data<PathBuf>) -> HttpResponse {
    let characters = filesystem::list_characters(&prefix);
    let encoded = bincode::serialize(&characters).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(encoded)
}

#[post("/read/{base_path:.+}")]
async fn read_character(
    web::Path(base_path): web::Path<PathBuf>,
    prefix: web::Data<PathBuf>,
) -> std::io::Result<HttpResponse> {
    let mut path = prefix.to_path_buf();
    path.push(base_path);
    let bytes = std::fs::read(path)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(bytes))
}

#[post("/write/{base_path:.+}")]
async fn write_character(
    web::Path(base_path): web::Path<PathBuf>,
    prefix: web::Data<PathBuf>,
    bytes: web::Bytes,
) -> std::io::Result<HttpResponse> {
    let mut path = prefix.to_path_buf();
    path.push(base_path);

    std::fs::write(path, bytes)?;

    Ok(HttpResponse::Ok().into())
}
