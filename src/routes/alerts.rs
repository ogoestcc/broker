use actix_web::{web, Scope};

use crate::{controllers::alerts, middlewares};

pub fn config(cfg: &mut web::ServiceConfig) {
    let factory: Scope<_> = web::scope("/alerts")
        .wrap(middlewares::auth::Auth)
        .service(alerts::recommendations::top_alerts)
        .service(alerts::recommendations::content_based);

    cfg.service(factory);
}
