mod logger;
mod services;

pub use logger::logger;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub port: u16,
    recommender: services::Service,
    database: services::Service,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    pub fn server_addr(&self) -> String {
        format!("[::1]:{}", self.port)
    }
}
