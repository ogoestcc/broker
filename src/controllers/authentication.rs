use std::sync::{Arc, Mutex};

use validator::Validate;

use actix_web::{error::Error as ActixError, post, web, HttpResponse};

use crate::{
    config::Config,
    grpc::{database::DatabaseService, recommender::RecommenderService},
    middlewares::validation::Validator,
    models::user::Claims,
    resources::errors::{auth::AuthError, ServiceError},
    utils::jwt,
};

#[derive(Debug, Validate, serde::Deserialize, serde::Serialize, Clone)]
pub struct LoginBody {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct LoginResponse {
    token: String,
}

#[post("/authentication")] // /api/authentication
pub async fn login(
    payload: Validator<web::Json<LoginBody>>,
    db: web::Data<Arc<Mutex<DatabaseService>>>,
    recommender: web::Data<Arc<Mutex<RecommenderService>>>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ActixError> {
    let auth = &config.auth;

    let mut db = db.lock().unwrap();
    let user = db
        .get_user_by_email(&payload.email)
        .await
        .map_err(AuthError::authentication_error)
        .map_err(ServiceError::from)?;
    drop(db); // unlock db mutex

    let valid_password = auth.verify_password(user.get_password(), payload.password.as_bytes());
    if !valid_password {
        Err(r"Incorrect Password".to_owned())
            .map_err(AuthError::authentication_error)
            .map_err(ServiceError::from)?;
    }

    let id = user.get_id() as u32;
    let recommender = recommender.lock().unwrap();
    async { recommender.load_user_data(id).await }.await?;

    let user_claims = Claims {
        id,
        email: user.get_email().into(),
        active: user.get_active(),
        exp: auth.get_token_exp(),
    };

    Ok(HttpResponse::Ok().json(LoginResponse {
        token: jwt::encode(user_claims, auth.secret_key.as_bytes()).unwrap(),
    }))
}
