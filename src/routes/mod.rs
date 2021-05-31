mod auth;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .wrap(crate::middlewares::auth::Auth)
            .configure(auth::config),
    );
}
