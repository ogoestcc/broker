use crate::resources::errors::InternalServerError;
use protos::recommender::{collaborative_filtering, content_based};

use super::{Error, RecommenderService};

impl RecommenderService {
    pub async fn content_based(&self, user_id: u32, n: Option<u32>) -> Result<Vec<String>, Error> {
        let mut request = content_based::Request::default();

        request.set_user_id(user_id);
        request.set_alerts_number(n.unwrap_or(20));

        let unary_receiver = self
            .recommender
            .content_based_async(&request)
            .map_err(InternalServerError::from)?;

        let response = unary_receiver.await.map_err(InternalServerError::from)?;

        Ok(response.alerts.into_vec())
    }

    pub async fn collaborative_filtering(
        &self,
        user_id: u32,
        n: Option<u32>,
    ) -> Result<Vec<String>, Error> {
        let mut request = collaborative_filtering::Request::default();

        request.set_user_id(user_id);
        request.set_alerts_number(n.unwrap_or(20));

        let unary_receiver = self
            .recommender
            .collaborative_filtering_async(&request)
            .map_err(InternalServerError::from)?;

        let response = unary_receiver.await.map_err(InternalServerError::from)?;

        Ok(response.alerts.into_vec())
    }
}
