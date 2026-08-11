use anyhow::Context;
use tonic::transport::Channel;
use homelab_proto::admin::console_file_service_client::ConsoleFileServiceClient;

pub struct FileClient {
    file: ConsoleFileServiceClient<Channel>,
}

impl FileClient {
    pub async fn connect(addr: String) -> anyhow::Result<Self> {
        let channel = Channel::from_shared(addr.clone())
            .with_context(|| format!("invalid server address: {addr}"))?
            .connect()
            .await
            .with_context(|| format!("could not reach admin-console at {addr}"))?;

        Ok(Self {
            file: ConsoleFileServiceClient::new(channel),
        })
    }
}