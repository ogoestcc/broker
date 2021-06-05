use crate::resources::errors::{InternalServerError, ServiceError};
use protos::{recommender, Client};

mod non_personalized;
mod personalized;

#[derive(Debug, Clone)]
pub struct RecommenderService {
    recommender: recommender::Recommender,
}

impl RecommenderService {
    pub async fn connect<U: ToString>(endpoint: U) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = endpoint.to_string();

        Ok(Self {
            recommender: recommender::Recommender::connect(endpoint.as_str()).await?,
        })
    }
}

type Error = ServiceError<InternalServerError>;
