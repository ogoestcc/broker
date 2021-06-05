use crate::resources::errors::InternalServerError;
use protos::{
    recommender::top,
    types::alerts::Alert,
};

use super::{Error, RecommenderService};

impl RecommenderService {
    pub async fn top_n(
        &self,
        n: Option<u32>,
        content: Option<String>,
    ) -> Result<Vec<Alert>, Error> {
        let mut request = top::Request::default();
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