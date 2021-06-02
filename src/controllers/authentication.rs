use std::sync::{Arc, Mutex};

use validator::Validate;

use actix_web::{error::Error as ActixError, post, web, HttpResponse};

use crate::{
    grpc::database::DatabaseService,
    middlewares::validation::Validator,
    resources::errors::{auth::AuthError, ServiceError},
};

#[derive(Debug, Validate, serde::Deserialize, serde::Serialize, Clone)]
pub struct LoginBody {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[post("/authentication")]
pub async fn login(
    body: Validator<LoginBody, web::Json<LoginBody>>,
    db: web::Data<Arc<Mutex<DatabaseService>>>,
) -> Result<HttpResponse, ActixError> {
    let mut db = db.lock().unwrap();
    let payload = body.into_inner();

    let user = db.get_user_by_email(&payload.email)
        .await?;

    println!("{:?}", user);

    // println!("{:?}", opt_flag); // test middleware

    // TODO create login

    Ok(HttpResponse::Ok().json(payload))
}
