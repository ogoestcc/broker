use std::ops::{Deref, DerefMut};

use crate::protos::recommender_grpc;

pub struct Recommender {
    client: recommender_grpc::RecommenderClient,
}

#[async_trait::async_trait]
impl crate::Client for Recommender {
    async fn connect(addr: &str) -> Result<Self, std::io::Error> {
        Ok(Recommender {
            client: recommender_grpc::RecommenderClient::new(Self::_connect(addr).await?),
        })
    }
}

impl Deref for Recommender {
    type Target = recommender_grpc::RecommenderClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for Recommender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
