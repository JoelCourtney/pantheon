pub mod routes;

use actix_web::{HttpServer};
use actix_cors::Cors;

pub fn new(port: u16) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        let cors = Cors::permissive();

        actix_web::App::new()
            .wrap(cors)
            .service(routes::list)
    })
        .bind(("127.0.0.1", port))?
        .run())
}
