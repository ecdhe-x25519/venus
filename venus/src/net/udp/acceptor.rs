use std::sync::Arc;

pub use crate::protocols::udp::configs::{ServerSide, UdpServerConfig, UdpCommonConfig};
pub use crate::protocols::udp::connection::UdpConnection;

use crate::error::UdpError;

pub struct UdpServer {
    pub(crate) config: Arc<UdpServerConfig>,
    pub(crate) connection: UdpConnection<ServerSide>,
    pub(crate) pre_init_conns: Option<Vec<UdpConnection<ServerSide>>>,
}

impl UdpServer {
    pub async fn bind(config: Arc<UdpServerConfig>) -> Result<Self, UdpError> {
        let connection: UdpConnection<ServerSide> = UdpConnection::bind(config.clone()).await?;

        let mut pre_init_conns: Option<Vec<UdpConnection<ServerSide>>> = None;

        if config.pre_init_conns {
            pre_init_conns = Some(connection.pre_init().await?);
        };

        Ok(Self {
            config,
            connection,
            pre_init_conns,
        })
    }
}