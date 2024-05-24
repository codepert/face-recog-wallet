use crate::controllers::controllers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(index)) // GET request to "/"
            .route("/status", web::get().to(status)) // GET request to "/status"
            .route("/get_wallet", web::post().to(get_wallet_post))
            .route("/create_wallet", web::post().to(create_wallet_post)) 
    );
}