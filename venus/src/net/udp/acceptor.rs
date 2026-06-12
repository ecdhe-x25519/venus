use std::sync::Arc;
use std::net::SocketAddr;

use tokio::sync::Semaphore;

use crate::error::UdpError;

use crate::protocols::udp::configs::UdpServerSide;
pub use crate::protocols::udp::configs::{UdpServerConfig, UdpCommonConfig};
pub use crate::protocols::udp::connection::UdpConnection;

use bytes::BytesMut;

pub struct UdpServer {
    pub(crate) _config: Arc<UdpServerConfig>,
    pub(crate) connection: UdpConnection<UdpServerSide>,
    pub(crate) semaphore: Option<Semaphore>,
}

impl UdpServer {
    pub async fn bind(config: Arc<UdpServerConfig>) -> Result<Self, UdpError> {
        let mut semaphore: Option<Semaphore> = None;        
        if config.connection_mode {
            semaphore = Some(Semaphore::new(config.max_conns.unwrap()));
        }

        let connection: UdpConnection<UdpServerSide> = UdpConnection::new(config.clone(), None).await?;

        Ok(Self {
            _config: config,
            connection,
            semaphore,
        })
    }

    pub fn handle(&mut self) -> &mut UdpConnection<UdpServerSide> {
        &mut self.connection
    }

    pub async fn recv(&mut self) -> Result<(BytesMut, SocketAddr), UdpError> {
        let (n, addr) = self.connection.recv().await?;
        Ok((n, addr))
    }

    pub async fn accept(&mut self) -> Result<(BytesMut, UdpConnection<UdpServerSide>), UdpError> {
        let _permit = self.semaphore.as_ref().unwrap().acquire().await
            .map_err(|e| UdpError::Std(format!("semaphore error: {e}")))?;
        
        let (data, conn) = self.connection.accept().await?;
        
        Ok((data, conn))
    }

    pub async fn send(&mut self, _data: &[u8], _peer_addr: Option<SocketAddr>) -> Result<(), UdpError> {
        self.connection.send(_peer_addr).await?;

        Ok(())
    }
}