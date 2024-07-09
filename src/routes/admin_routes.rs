use actix_web::web;
use crate::controllers::{admin_order_controller, admin_user_controller, admin_product_controller};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/users", web::get().to(admin_user_controller::get_users))
            .route("/user/{id}", web::get().to(admin_user_controller::get_user))
            .route("/orders", web::get().to(admin_order_controller::get_orders))
            .route("/order/{id}", web::get().to(admin_order_controller::get_order))
            .route("/order/{id}", web::put().to(admin_order_controller::update_order))
            .route("/order/{id}", web::delete().to(admin_order_controller::delete_order))
            .route("/products", web::post().to(admin_product_controller::create_product))
            .route("/product/{id}", web::put().to(admin_product_controller::update_product))
            .route("/product/{id}", web::delete().to(admin_product_controller::delete_product))
    );
}