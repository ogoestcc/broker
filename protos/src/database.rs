use crate::protos::{
    alerts_grpc::AlertsClient, ratings_grpc::RatingsClient, users_grpc::UsersClient,
};


#[derive(Clone)]
pub struct Database {
    pub alerts: AlertsClient,
    pub users: UsersClient,
    pub ratings: RatingsClient,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database {{ ... }}")
    }
}

#[async_trait::async_trait]
impl crate::Client for Database {
    async fn connect(addr: &str) -> Result<Self, std::io::Error> {
        let channel = Self::_connect(addr).await?;

        Ok(Database {
            users: UsersClient::new(channel.clone()),
            alerts: AlertsClient::new(channel.clone()),
            ratings: RatingsClient::new(channel),
        })
    }
}
