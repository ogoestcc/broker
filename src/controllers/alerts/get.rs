use std::sync::{Arc, Mutex};

use actix_web::{error::Error as ActixError, get, web, HttpResponse};

use crate::{
    grpc::database::DatabaseService,
    middlewares::validation::Validator,
    models::{alerts::Alert, user::Claims},
};

type Database = Arc<Mutex<DatabaseService>>;

#[derive(Debug, Default, validator::Validate, serde::Deserialize)]
#[serde(default)]
pub struct Filter {
    viewed: Option<bool>,
    favorited: Option<bool>,
}

#[get("{_:/?}")] // /api/alerts/
pub async fn get_alerts(
    query: Validator<web::Query<Filter>>,
    user: web::ReqData<Claims>,
    database: web::Data<Database>,
) -> Result<HttpResponse, ActixError> {
    let Filter { viewed, favorited } = query.into_inner();

    let mut database = database.lock().unwrap();

    let proto_alerts = if let Some(true) = viewed {
        database.get_viewed_alerts(user.id, favorited).await?
    } else {
        database.get_alerts().await?
    };

    drop(database); // unlock before mapping

    Ok(HttpResponse::Ok().json(proto_alerts.iter().map(Alert::from).collect::<Vec<_>>()))
}
