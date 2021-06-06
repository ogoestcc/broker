mod alerts;
mod auth;
mod users;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(alerts::config)
            .configure(auth::config)
            .configure(users::config),
    );
}
