use actix_web::web;
use crate::controllers::user_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(user_controller::get_user)),
    );
}