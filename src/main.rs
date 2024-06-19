mod controllers;
mod routes;
mod dto;
mod utils;
mod database;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .configure(routes::user_routes::config)
            .configure(routes::product_routes::config)
            .configure(routes::order_routes::config)
            .configure(routes::auth_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}