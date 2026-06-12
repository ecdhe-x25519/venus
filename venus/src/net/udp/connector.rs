use std::sync::Arc;

use crate::protocols::udp::configs::UdpClientSide;
pub use crate::protocols::udp::configs::{UdpClientConfig, UdpCommonConfig};
pub use crate::protocols::udp::connection::UdpConnection;

use crate::error::UdpError;

pub struct UdpClient {
    pub(crate) _config: Arc<UdpClientConfig>,
    pub(crate) connection: UdpConnection<UdpClientSide>,
}

impl UdpClient {
    pub async fn connect(config: Arc<UdpClientConfig>) -> Result<Self, UdpError> {
        let connection: UdpConnection<UdpClientSide> = UdpConnection::new(config.clone(), Some(config.remote_addr)).await?;

        Ok(Self {
            _config: config,
            connection,
        })
    }

    pub fn handle(&mut self) -> &mut UdpConnection<UdpClientSide> {
        &mut self.connection
    }
}