use actix_web::{App, HttpServer};
mod controllers;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::user_routes::config)
            .configure(routes::product_routes::config)
            .configure(routes::order_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}