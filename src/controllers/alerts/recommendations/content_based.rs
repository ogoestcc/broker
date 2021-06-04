
use std::sync::{Arc, Mutex};

use validator::Validate;

use actix_web::{error::Error as ActixError, get, web, HttpResponse};

use crate::{grpc::recommender::RecommenderService, middlewares::validation::Validator, models::{alerts::Alert, user::Claims}};

#[derive(Debug, Validate, serde::Deserialize, Clone)]
pub struct ContentBased {
    #[validate(range(min = 1))]
    #[serde(default)]
    n: Option<u32>,
}

#[get("/content_based")] // /api/alerts/content_based
pub async fn content_based(
    query: Validator<ContentBased, web::Query<ContentBased>>,
    user: web::ReqData<Claims>,
    recommender: web::Data<Arc<Mutex<RecommenderService>>>,
) -> Result<HttpResponse, ActixError> {
    let query = query.into_inner();

    let recommender = recommender.lock().unwrap();

    let proto_alerts = recommender.content_based(user.id, query.n).await?;

    drop(recommender); // unlock before mapping

    Ok(HttpResponse::Ok().json(proto_alerts.iter().map(Alert::from).collect::<Vec<_>>()))
}
