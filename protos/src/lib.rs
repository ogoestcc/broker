use std::{io::{Error, ErrorKind}, sync::Arc, time::Duration};

use grpcio::{Channel, ChannelBuilder, EnvBuilder};

pub mod database;
pub mod recommender;
pub mod types;

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

#[async_trait::async_trait]
pub trait Client: Sized {
    async fn connect(addr: &str) -> Result<Self, Error>;

    async fn _connect(addr: &str) -> Result<Channel, Error> {

        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(addr);

        if !ch.wait_for_connected(Duration::from_secs(5)).await {
            return Err(Error::new(
                ErrorKind::ConnectionRefused,
                format!("Unable to connect with grpc server: {}", addr),
            ));
        }

        Ok(ch)
    }
}
