use actix_web::web;
use crate::controllers::product_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(product_controller::get_products))
            .route("", web::post().to(product_controller::create_product))
    );
}