use std::sync::Arc;

pub use crate::protocols::udp::configs::{ClientSide, UdpClientConfig, UdpCommonConfig};
pub use crate::protocols::udp::connection::UdpConnection;

use crate::error::UdpError;

pub struct UdpClient {
    pub(crate) config: Arc<UdpClientConfig>,
    pub(crate) connection: UdpConnection<ClientSide>,
}

impl UdpClient {
    pub async fn connect(config: Arc<UdpClientConfig>) -> Result<Self, UdpError> {
        let mut connection: UdpConnection<ClientSide> = UdpConnection::bind(config.clone()).await?;
        connection.connect(config.remote_addr).await?;

        Ok(Self {
            config,
            connection,
        })
    }

    pub async fn handle(&self) -> &UdpConnection<ClientSide> {
        &self.connection
    }
}