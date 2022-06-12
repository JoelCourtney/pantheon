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
use pantheon::shared::Query;
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
            .service(serve_system)
            .service(post_queries)
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
async fn serve_root(root: web::Data<ServeRoot>) -> std::io::Result<HttpResponse> {
    let root = root.into_inner();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read(format!("{root}/home/index.html"))?))
}

/// Serves just the favicon.
#[get("/favicon.ico")]
async fn serve_icon(root: web::Data<ServeRoot>) -> std::io::Result<HttpResponse> {
    let root = root.into_inner();
    let path = format!("{root}/favicon.ico");
    Ok(HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(std::fs::read(&path)?))
}

#[get("/systems/{system}/{file:.*}")]
async fn serve_system(
    root: web::Data<ServeRoot>,
    path: web::Path<(String, String)>,
) -> std::io::Result<HttpResponse> {
    let (system, file) = path.into_inner();
    let root = root.into_inner();
    let (path, mime) = if file.is_empty() || file == "/" {
        (
            format!("{root}/systems/{system}/index.html"),
            ContentType::html().to_string(),
        )
    } else {
        (
            format!("{root}/systems/{system}/{file}"),
            mime_guess::from_path(Path::new(&file))
                .first()
                .unwrap()
                .essence_str()
                .to_string(),
        )
    };
    Ok(HttpResponse::Ok()
        .content_type(mime)
        .body(std::fs::read(&(path)).unwrap()))
}

/// Serves files for the home page, before a system/character is chosen.
#[get("/{file}")]
async fn serve_home(
    root: web::Data<ServeRoot>,
    file: web::Path<String>,
) -> std::io::Result<HttpResponse> {
    let root = root.into_inner();
    let file = &file.into_inner();
    let path = format!("{root}/home/{file}");
    Ok(HttpResponse::Ok()
        .content_type(
            mime_guess::from_path(Path::new(file))
                .first()
                .unwrap()
                .essence_str(),
        )
        .body(std::fs::read(&path)?))
}

#[post("/query")]
async fn post_queries(
    bytes: web::Bytes,
    prefix: web::Data<PathBuf>,
    root: web::Data<ServeRoot>,
) -> std::io::Result<HttpResponse> {
    let query: Query = bincode::deserialize(&bytes).unwrap();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream().essence_str())
        .body(match query {
            Query::ListCharacters => {
                bincode::serialize(&filesystem::list_characters(&prefix)).unwrap()
            }
            Query::ListSystems => {
                bincode::serialize(&filesystem::list_systems(root.get_ref())).unwrap()
            }
            Query::ReadCharacter(base_path) => {
                let mut path = prefix.to_path_buf();
                path.push(base_path);
                std::fs::read(path)?
            }
            Query::WriteCharacter(base_path, bytes) => {
                let mut path = prefix.to_path_buf();
                path.push(base_path);
                std::fs::write(path, bytes)?;
                bincode::serialize(&()).unwrap()
            }
        }))
}
