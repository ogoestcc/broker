use actix_web::{web, Scope};

use crate::{
    controllers::alerts::{self, recommendations},
    middlewares,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let factory: Scope<_> = web::scope("/alerts")
        .wrap(middlewares::authorization::Authorization)
        .service(alerts::get_alerts)
        .service(recommendations::non_personalized::top_alerts) //* GET
        .service(recommendations::personalized::content_based) //* GET
        .service(recommendations::personalized::collaborative_filtering); //* GET

    cfg.service(factory);
}
