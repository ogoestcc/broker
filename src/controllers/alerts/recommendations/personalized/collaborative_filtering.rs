use std::sync::{Arc, Mutex};

use actix_web::{error::Error as ActixError, get, web, HttpResponse};

use crate::{
    grpc::{database::DatabaseService, recommender::RecommenderService},
    middlewares::validation::Validator,
    models::{alerts::Alert, user::Claims},
};

use super::Request;

type Recommender = Arc<Mutex<RecommenderService>>;

#[get("/collaborative_filtering")] // /api/alerts/collaborative_filtering
pub async fn collaborative_filtering(
    query: Validator<web::Query<Request>>,
    user: web::ReqData<Claims>,
    recommender: web::Data<Recommender>,
    database: web::Data<Arc<Mutex<DatabaseService>>>,
) -> Result<HttpResponse, ActixError> {
    let query = query.into_inner();

    let recommender = recommender.lock().unwrap();
    let proto_alerts =
        async { recommender.collaborative_filtering(user.id, query.n).await }.await?;

    let mut database = database.lock().unwrap();
    let proto_alerts = async { database.get_alerts_by_ids(proto_alerts).await }.await?;

    Ok(HttpResponse::Ok().json(proto_alerts.iter().map(Alert::from).collect::<Vec<_>>()))
}
