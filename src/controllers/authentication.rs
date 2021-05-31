use validator::Validate;

use actix_web::{HttpResponse, error::Error as ActixError, post, web};

use crate::middlewares::validation::Validator;

#[derive(Debug, Validate, serde::Deserialize, serde::Serialize)]
pub struct LoginBody {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}


#[post("/authentication")]
pub async fn login(body: Validator<LoginBody>, opt_flag: Option<web::ReqData<u32>>) -> Result<HttpResponse, ActixError> {
    let payload = body.into_inner();

    println!("{:?}", opt_flag); // test middleware

    // TODO create login

    Ok(HttpResponse::Ok().json(payload))
}


