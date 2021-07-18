use crate::resources::errors::{InternalServerError, ServiceError};
use protos::recommender::load_data;
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

    pub async fn load_user_data(&self, user_id: u32) -> Result<(), Error> {
        let mut request = load_data::Request::default();
        request.set_user_id(user_id);

        let unary_receiver = self
            .recommender
            .load_user_data_async(&request)
            .map_err(InternalServerError::from)?;

        unary_receiver.await.map_err(InternalServerError::from)?;

        Ok(())
    }
}

type Error = ServiceError<InternalServerError>;
