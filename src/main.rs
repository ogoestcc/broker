mod config;
mod controllers;
mod grpc;
mod middlewares;
mod models;
mod resources;
mod routes;
mod utils;

use std::sync::{Arc, Mutex};

use actix_web::{
    http::header,
    middleware::{normalize::TrailingSlash, DefaultHeaders, NormalizePath},
    App, HttpServer,
};

use env_logger::Env;

use grpc::{database::DatabaseService, recommender::RecommenderService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(r"info,broker")).init();
    dotenv::dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let addr = config.server_addr();

    let db_service = Arc::new(Mutex::new(
        DatabaseService::connect(config.database.to_string())
            .await
            .unwrap(),
    ));

    let recommender = Arc::new(Mutex::new(
        RecommenderService::connect(config.recommender.to_string())
            .await
            .unwrap(),
    ));

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(DefaultHeaders::new().header(header::CONTENT_TYPE, r"application/json"))
            .wrap(config::logger())
            .data(config.clone())
            .data(db_service.clone())
            .data(recommender.clone())
            .configure(routes::config)
    })
    .bind(addr)?
    .run()
    .await
}
