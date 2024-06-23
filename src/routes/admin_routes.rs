pub fn config(cfg: &mut: ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/product", web::post().to(admin_product_controller::create_product))
            .route("/product/{id}", web::put().to(admin_product_controller::update_product))
            .route("/product/{id}", web::delete().to(admin_product_controller::delete_product))
            .route("/orders", web::get().to(admin_order_controller::get_orders))
            .route("/order/{id}", web::get().to(admin_order_controller::get_order))
            .route("/order/{id}", web::put().to(admin_order_controller::update_order))
            .route("/order/{id}", web::delete().to(admin_order_controller::delete_order))
    )
}