use crate::resources::errors::{InternalServerError, ServiceError};
use protos::{recommender, types::alerts::Alert, Client};

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

impl RecommenderService {
    pub async fn top_n(
        &self,
        n: Option<u32>,
        content: Option<String>,
    ) -> Result<Vec<Alert>, ServiceError<InternalServerError>> {
        let mut request = recommender::top::Request::default();
        request.set_alerts_number(n.unwrap_or(20));

        if let Some(content) = content {
            request.set_content(content);
        }

        let unary_receiver = self
            .recommender
            .top_n_async(&request)
            .map_err(InternalServerError::from)?;

        let response = unary_receiver.await.map_err(InternalServerError::from)?;

        Ok(response.alerts.into_vec())
    }
}
