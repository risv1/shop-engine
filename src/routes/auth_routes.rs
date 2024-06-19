use actix_web::web;
use crate::controllers::auth_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth_controller::register))
            .route("/login", web::post().to(auth_controller::login))
            .route("/logout", web::post().to(auth_controller::logout)),
    );
}