use actix_web::web;
use crate::controllers::order_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/cart", web::get().to(order_controller::get_cart))
            .route("/cart", web::post().to(order_controller::add_to_cart))
            .route("", web::get().to(order_controller::get_orders))
            .route("", web::post().to(order_controller::create_order))
            .route("/{id}", web::get().to(order_controller::get_order))
            .route("/{id}", web::put().to(order_controller::update_order))
            .route("/{id}", web::delete().to(order_controller::delete_order)),
    );
}