use actix_web::web;

use crate::controllers::authentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(authentication::login);
}
