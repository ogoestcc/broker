use std::ops::{Deref, DerefMut};

use crate::protos::recommender_grpc;

pub mod top {
    pub use crate::protos::recommender::{TopN_Request as Request, TopN_Response as Response};
}

pub mod collaborative_filtering {
    pub use crate::protos::recommender::{
        CollaborativeFiltering_Request as Request, CollaborativeFiltering_Response as Response,
    };
}

pub mod content_based {
    pub use crate::protos::recommender::{
        ContentBased_Request as Request, ContentBased_Response as Response,
    };
}

#[derive(Clone)]
pub struct Recommender {
    client: recommender_grpc::RecommenderClient,
}

impl std::fmt::Debug for Recommender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Recommender {{ ... }}")
    }
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
