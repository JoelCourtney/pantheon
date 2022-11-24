pub mod routes;

use actix_web::{HttpServer};
use actix_web::dev::Server;

pub fn new(port: u16) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        actix_web::App::new()
            .service(routes::health)
    })
        .bind(("127.0.0.1", port))?
        .run())
}
