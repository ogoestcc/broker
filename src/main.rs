mod config;
mod controllers;
mod grpc;
mod middlewares;
mod models;
mod resources;
mod routes;
mod utils;

use protos::Client;
use std::sync::{Arc, Mutex};

use actix_web::{
    http::header,
    middleware::{normalize::TrailingSlash, DefaultHeaders, NormalizePath},
    App, HttpServer,
};

use env_logger::Env;
// use grpc::database::DatabaseService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();
    let addr = config.server_addr();

    let database_config = &config.database;

    let database_addr = format!("{}:{}", database_config.host, database_config.port);

    env_logger::Builder::from_env(Env::default().default_filter_or(r"info,broker")).init();

    let db_service = Arc::new(Mutex::new(
        grpc::database::DatabaseService::connect(database_addr.clone())
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
            .configure(routes::config)
    })
    .bind(addr)?
    .run()
    .await
}
