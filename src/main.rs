mod config;
mod controllers;
mod middlewares;
mod models;
mod resources;
mod routes;
mod utils;

use actix_web::{
    http::header,
    middleware::{normalize::TrailingSlash, DefaultHeaders, NormalizePath},
    App, HttpServer,
};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();
    let addr = config.server_addr();

    env_logger::Builder::from_env(Env::default().default_filter_or(r"info,broker")).init();

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(DefaultHeaders::new().header(header::CONTENT_TYPE, r"application/json"))
            .wrap(config::logger())
            .configure(routes::config)
    })
    .bind(addr)?
    .run()
    .await
}
