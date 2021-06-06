use actix_web::{web, Scope};

use crate::{controllers::users, middlewares};

pub fn config(cfg: &mut web::ServiceConfig) {
    let factory: Scope<_> = web::scope("/users")
        .service(users::create) //* POST /api/users/
        .service(web::scope("/{user_id}").wrap(middlewares::authorization::Authorization));

    cfg.service(factory);
}
