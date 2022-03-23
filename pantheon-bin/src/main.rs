//! The server for Pantheon.
//! 
//! All things considered, its actually pretty simple.
//! It serves files over `GET` out of `PANTHEON_ROOT/www`,
//! and exposes a few `POST` endpoints for listing, reading,
//! and writing character files.

mod filesystem;

use actix_web::http::header::ContentType;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use colored::Colorize;
use filesystem::ServeRoot;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let prefix = match opt.dir {
        Some(path) => path,
        None => std::env::current_dir()?,
    };
    let root = filesystem::ServeRoot::default();
    println!(
        "Serving {} on {}{}",
        prefix.as_path().to_str().unwrap().green(),
        "http://localhost:".green(),
        opt.port.to_string().green()
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(prefix.clone()))
            .app_data(web::Data::new(root.clone()))
            .wrap(middleware::Compress::default())
            .service(serve_root)
            .service(serve_icon)
            .service(serve_home)
            .service(list_characters)
            .service(read_character)
            .service(write_character)
    })
    .bind(format!("127.0.0.1:{}", opt.port))?
    .run()
    .await
}

/// Server for Pantheon.
///
/// Serves an rpg-system-agnostic home page at http://localhost:<port>
/// and links to character viewers in any implemented systems.
#[derive(StructOpt, Debug)]
#[structopt(name = "pantheon")]
struct Opt {
    /// Path prefix to serve characters from (optional).
    #[structopt(short, long, parse(from_os_str))]
    dir: Option<PathBuf>,

    /// Port to host site on.
    #[structopt(short, long, default_value = "8080")]
    port: u32,
}

/// Serves the index page.
#[get("/")]
async fn serve_root(root: web::Data<ServeRoot>) -> HttpResponse {
    let root = root.into_inner();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read(format!("{root}/home/index.html")).unwrap())
}

/// Serves just the favicon.
#[get("/icon.png")]
async fn serve_icon(root: web::Data<ServeRoot>) -> HttpResponse {
    let root = root.into_inner();
    let path = format!("{root}/icon.png");
    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(std::fs::read(&path).expect(&format!("file not found: {path}")))
}

/// Serves files for the home page, before a system/character is chosen.
#[get("/{file}")]
async fn serve_home(root: web::Data<ServeRoot>, file: web::Path<String>) -> HttpResponse {
    let root = root.into_inner();
    let file = &file.into_inner();
    let path = format!("{root}/home/{file}");
    HttpResponse::Ok()
        .content_type(
            mime_guess::from_path(Path::new(file))
                .first()
                .unwrap()
                .essence_str(),
        )
        .body(std::fs::read(&path).expect(&format!("file not found: {path}")))
}

/// Serves a list of all characters found in this directory.
#[post("/list_characters")]
async fn list_characters(prefix: web::Data<PathBuf>) -> HttpResponse {
    let characters = filesystem::list_characters(&prefix);
    let encoded = bincode::serialize(&characters).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(encoded)
}

/// Serves the raw bytes of a character file.
#[post("/read_character/{base_path:.+}")]
async fn read_character(
    base_path: web::Path<PathBuf>,
    prefix: web::Data<PathBuf>,
) -> std::io::Result<HttpResponse> {
    let mut path = prefix.to_path_buf();
    path.push(base_path.into_inner());
    let bytes = std::fs::read(path)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(bytes))
}

/// Writes the raw bytes of a character to file.
#[post("/write_character/{base_path:.+}")]
async fn write_character(
    base_path: web::Path<PathBuf>,
    prefix: web::Data<PathBuf>,
    bytes: web::Bytes,
) -> std::io::Result<HttpResponse> {
    let mut path = prefix.to_path_buf();
    path.push(base_path.into_inner());

    std::fs::write(path, bytes)?;

    Ok(HttpResponse::Ok().into())
}
