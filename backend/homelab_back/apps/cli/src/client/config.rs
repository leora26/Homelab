use tonic::transport::Channel;
use anyhow::{Context, Result};

pub async fn connect(addr: String) -> Result<Channel> {
    Channel::from_shared(addr.clone())
        .with_context(|| format!("invalid server address: {addr}"))?
        .connect()
        .await
        .with_context(|| format!("could not reach admin-console at {addr}"))
}