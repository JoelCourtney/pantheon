mod filesystem;

use actix_web::http::header::ContentType;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use std::path::{Path, PathBuf};
use colored::Colorize;
use structopt::StructOpt;
use filesystem::PantheonRoot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let prefix = match opt.dir {
        Some(path) => path,
        None => std::env::current_dir()?,
    };
    let pantheon_root = filesystem::pantheon_root().unwrap();
    println!("Serving {} on {}{}", prefix.as_path().to_str().unwrap().green(), "http://localhost:".green(), opt.port.to_string().green());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(prefix.clone()))
            .app_data(web::Data::new(pantheon_root.clone()))
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

#[get("/")]
async fn serve_root(root: web::Data<PantheonRoot>) -> HttpResponse {
    let root = root.into_inner();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read(format!("{root}/client_home/dist/index.html")).unwrap())
}

#[get("/icon.png")]
async fn serve_icon(root: web::Data<PantheonRoot>) -> HttpResponse {
    let root = root.into_inner();
    let path = format!("{root}/client_home/public/icon.png");
    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(std::fs::read(&path).expect(&format!("file not found: {path}")))
}

#[get("/{file}")]
async fn serve_home(root: web::Data<PantheonRoot>, file: web::Path<String>) -> HttpResponse {
    let root = root.into_inner();
    let file = &file.into_inner();
    let path = format!("{root}/client_home/dist/{file}");
    HttpResponse::Ok()
        .content_type(
            mime_guess::from_path(Path::new(file))
                .first().unwrap()
                .essence_str()
        )
        .body(std::fs::read(&path).expect(&format!("file not found: {path}")))
}

#[post("/list_characters")]
async fn list_characters(prefix: web::Data<PathBuf>) -> HttpResponse {
    let characters = filesystem::list_characters(&prefix);
    let encoded = bincode::serialize(&characters).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(encoded)
}

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
