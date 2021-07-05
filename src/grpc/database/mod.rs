mod alerts;
mod users;

use protos::{database, Client};

#[derive(Debug, Clone)]
pub struct DatabaseService {
    db: database::Database,
}

impl DatabaseService {
    pub async fn connect<U: ToString>(endpoint: U) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = endpoint.to_string();

        Ok(Self {
            db: database::Database::connect(endpoint.as_str()).await?,
        })
    }
}
