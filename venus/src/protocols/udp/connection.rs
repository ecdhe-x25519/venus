use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::timeout;

use super::configs::UdpCommonConfig;

use crate::error::UdpError;

use bytes::*;

pub struct UdpConnection {
    config: Arc<UdpCommonConfig>,
    read_buf: BytesMut,
    write_buf: BytesMut,
    peer_addr: SocketAddr,
    socket: Arc<UdpSocket>
}

impl UdpConnection {
    pub async fn new(socket: Arc<UdpSocket>, config: Arc<UdpCommonConfig>) -> Result<Self, UdpError> {
        let peer_addr: SocketAddr = socket.peer_addr()
            .map_err(|e| UdpError::Std(format!("get socket addr error: {e}")))?;

        let capacity: usize = config.buffers_capacity;

        Ok(Self {
            config,
            read_buf: BytesMut::with_capacity(capacity),
            write_buf: BytesMut::with_capacity(capacity),
            peer_addr,
            socket,
        })
    }

    pub async fn recv_all(&mut self) -> Result<> {
        
    }

    pub async fn recv_from(&mut self, addr: SocketAddr) -> Result<> {
        self.socket.recv
    }

    pub async fn send_to(&mut self, addr: SocketAddr) -> Result<> {

    }
}