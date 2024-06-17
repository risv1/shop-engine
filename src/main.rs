use actix_web::web;
use actix_web::{App, HttpServer};
mod controllers;
mod routes;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    HttpServer::new(|| {
        App::new()
            .data(pool.clone())
            .configure(routes::user_routes::config)
            .configure(routes::product_routes::config)
            .configure(routes::order_routes::config)
            .configure(|cfg| routes::auth_routes::config(cfg, web::Data::new(pool.clone())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}