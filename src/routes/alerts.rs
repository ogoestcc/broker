use actix_web::{web, Scope};

use crate::{controllers::alerts::recommendations, middlewares};

pub fn config(cfg: &mut web::ServiceConfig) {
    let factory: Scope<_> = web::scope("/alerts")
        .wrap(middlewares::auth::Auth)
        .service(recommendations::non_personalized::top_alerts)           //* GET
        .service(recommendations::personalized::content_based)            //* GET
        .service(recommendations::personalized::collaborative_filtering); //* GET

    cfg.service(factory);
}
