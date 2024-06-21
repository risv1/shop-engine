use actix_web::web;
use crate::controllers::order_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/cart", web::get().to(order_controller::get_cart))
            .route("/cart", web::post().to(order_controller::add_to_cart))
            .route("/cart", web::delete().to(order_controller::delete_from_cart))
            .route("", web::get().to(order_controller::get_orders))
            .route("", web::post().to(order_controller::create_order))
            .route("/order", web::get().to(order_controller::get_order))
            .route("user_order", web::get().to(order_controller::get_user_order))
            .route("user_order", web::delete().to(order_controller::delete_user_order))
    );
}