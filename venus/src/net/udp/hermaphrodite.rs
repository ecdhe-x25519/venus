use std::net::SocketAddr;

use crate::protocols::udp::configs::*;
use crate::protocols::udp::connection::*;

use tokio::net::UdpSocket;

use crate::error::UdpError;

pub struct UdpHermaphrodite {
    config: UdpCommonConfig,
    socket: UdpSocket,
}

impl UdpHermaphrodite {
    pub async fn connect(&mut self, addr: SocketAddr) -> Result<(), UdpError> {
        self.socket.connect(addr)
            .await
            .map_err(|e| UdpError::Std(format!("connect error: {e}")))
    }

    pub async fn send() -> Result<> {

    }

    pub async fn recv() -> Result<> {

    }

    pub async fn bind(config: UdpCommonConfig) -> Result<Self, UdpError> {
        let socket = UdpSocket::bind(config.bind_addr)
            .await
            .map_err(|e| UdpError::Std(format!("bind error: {e}")))?;

        Ok(Self {
            config,
            socket,
        })
    }

    pub async fn send_to(&mut self, addr: SocketAddr) -> Result<> {
        self.socket.send_to(buf, addr)
    }

    pub async fn recv_from(&mut self) -> Result<> {
        
    }
}