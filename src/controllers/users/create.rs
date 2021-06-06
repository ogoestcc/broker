use std::sync::{Arc, Mutex};

use validator::Validate;

use actix_web::{error::Error as ActixError, post, web, HttpResponse};

use crate::{
    config::Config,
    grpc::database::DatabaseService,
    middlewares::validation::Validator,
    models::user::{Claims, User},
    utils::jwt,
};

#[derive(Debug, Validate, serde::Deserialize, Clone)]
pub struct Body {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Response {
    created: User,
    token: String,
}

#[post("/")] // /api/users/
pub async fn create_user(
    body: Validator<web::Json<Body>>,
    db: web::Data<Arc<Mutex<DatabaseService>>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ActixError> {
    let body = body.into_inner();

    let auth = &config.auth;

    let pass_hashed = auth.hash_password(body.password.as_str());

    let db = db.lock().unwrap();
    let created = db.create_user(body.email, pass_hashed).await?;
    drop(db); // unlock before mapping

    let user_claims = Claims {
        id: created.get_id() as u32,
        email: created.get_email().into(),
        active: created.get_active(),
        exp: auth.get_token_exp(),
    };

    Ok(HttpResponse::Created().json(Response {
        created: User::from(created),
        token: jwt::encode(user_claims, auth.secret_key.as_bytes()).unwrap(),
    }))
}
