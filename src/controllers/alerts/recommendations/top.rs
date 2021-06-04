use std::sync::{Arc, Mutex};

use validator::Validate;

use actix_web::{error::Error as ActixError, get, web, HttpResponse};

use crate::{
    grpc::recommender::RecommenderService,
    middlewares::validation::Validator,
    models::alerts::Alert,
    resources::errors::{InternalServerError, ServiceError},
};

fn n_default() -> Option<u32> {
    Some(20)
}

#[derive(Debug, Validate, serde::Deserialize, Clone)]
pub struct TopN {
    #[validate(range(min = 1))]
    #[serde(default = "n_default")]
    n: Option<u32>,
    #[validate(length(min = 1))]
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct LoginResponse {
    token: String,
}

#[get("/top")]
pub async fn top_alerts(
    query: Validator<TopN, web::Query<TopN>>,
    recommender: web::Data<Arc<Mutex<RecommenderService>>>,
) -> Result<HttpResponse, ActixError> {
    let query = query.into_inner();

    let recommender = recommender.lock().unwrap();

    let proto_alerts = recommender
        .top_n(query.n, query.content)
        .await
        .map_err(|err| ServiceError::new(InternalServerError(Some(err.to_string()))))?;

    drop(recommender); // unlock before mapping

    Ok(HttpResponse::Ok().json(proto_alerts.iter().map(Alert::from).collect::<Vec<_>>()))
}
