mod alerts;
mod auth;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::config)
            .configure(alerts::config),
    );
}
