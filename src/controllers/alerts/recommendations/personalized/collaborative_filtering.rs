use std::sync::{Arc, Mutex};

use actix_web::{error::Error as ActixError, get, web, HttpResponse};

use crate::{
    grpc::recommender::RecommenderService,
    middlewares::validation::Validator,
    models::{alerts::Alert, user::Claims},
};

use super::Request;

type Recommender = Arc<Mutex<RecommenderService>>;

#[get("/collaborative_filtering")] // /api/alerts/collaborative_filtering
pub async fn collaborative_filtering(
    query: Validator<Request, web::Query<Request>>,
    user: web::ReqData<Claims>,
    recommender: web::Data<Recommender>,
) -> Result<HttpResponse, ActixError> {
    let query = query.into_inner();

    let recommender = recommender.lock().unwrap();

    let proto_alerts = recommender.collaborative_filtering(user.id, query.n).await?;

    drop(recommender); // unlock before mapping

    Ok(HttpResponse::Ok().json(proto_alerts.iter().map(Alert::from).collect::<Vec<_>>()))
}