mod config;

use actix_web::{http::header, middleware::DefaultHeaders, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();
    let addr = config.server_addr();

    env_logger::Builder::from_env(Env::default().default_filter_or(r"info,broker")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().header(header::CONTENT_TYPE, r"application/json"))
            .wrap(config::logger())
    })
    .workers(1)
    .bind(addr)?
    .run()
    .await
}
