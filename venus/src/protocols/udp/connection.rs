use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::time::{Duration, timeout};

use super::configs::*;

use crate::error::UdpError;

use bytes::*;

pub struct UdpConnection<S: Side> {
    pub(crate) config: Arc<S::Config>,
    pub(crate) peer_addr: Option<SocketAddr>,
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) read_buf: BytesMut,
    pub(crate) write_buf: BytesMut,
}

impl<S: Side> UdpConnection<S> {
    pub(crate) async fn bind(side_config: Arc<S::Config>) -> Result<Self, UdpError> {
        let config: Arc<UdpCommonConfig> = S::common(side_config.clone());

        let capacity: usize = config.buffers_capacity;

        let socket: UdpSocket = UdpSocket::bind(config.bind_addr)
            .await
            .map_err(|e| UdpError::Std(format!("bind error: {e}")))?;

        socket.set_broadcast(config.enable_broadcast)
            .map_err(|e| UdpError::Std(format!("set broadcast error: {e}")))?;

        socket.bind_device(config.device.as_deref())
            .map_err(|e| UdpError::Std(format!("bind device error: {e}")))?;

        let mut peer_addr: Option<SocketAddr> = None;
        if S::is_client() {
            peer_addr = Some(config.bind_addr);
        }

        Ok(Self {
            config: side_config,
            peer_addr,
            socket: Arc::new(socket),
            read_buf: BytesMut::with_capacity(capacity),
            write_buf: BytesMut::with_capacity(capacity),
        })
    }
}

impl UdpConnection<ServerSide> {
    pub(crate) async fn pre_init(&self) -> Result<Vec<Self>, UdpError> {
        let max_conns: usize = self.config.max_conns.unwrap();

        let mut conns: Vec<UdpConnection<ServerSide>> = Vec::new();

        for _ in 0..max_conns {
            let conn: UdpConnection<ServerSide> = UdpConnection::bind(self.config.clone())
                .await
                .map_err(|e| UdpError::Std(format!("pre init conn error: {e}")))?;

            conns.push(conn);
        }

        Ok(conns)
    }

    pub async fn write_buf(&mut self, data: &[u8]) {
        self.write_buf.put(data);
    }

    pub async fn send_to(&mut self) -> Result<usize, UdpError> {
        let duration: Duration = self.config.common.send_timeout_secs;
        let write_buf: &mut BytesMut = &mut self.write_buf;

        if !self.peer_addr.is_some() { 
            return Err(UdpError::Std(format!("there is no send addr")))
        };

        let addr: SocketAddr = self.peer_addr.unwrap();

        let n: usize = timeout(duration, self.socket.send_to(write_buf, addr))
            .await
            .map_err(|e| UdpError::Timeout(format!("send, elapsed: {e}")))?
            .map_err(|e| UdpError::Std(format!("send error: {e}")))?;

        Ok(n)
    }

    pub async fn recv_from(&mut self) -> Result<usize, UdpError> {
        let duration: Duration = self.config.common.recv_timeout_secs;
        let read_buf: &mut BytesMut = &mut self.read_buf;

        let (n, addr) = timeout(duration, self.socket.recv_from(read_buf))
            .await
            .map_err(|e| UdpError::Timeout(format!("recv from, elapsed: {e}")))?
            .map_err(|e| UdpError::Std(format!("recv from error: {e}")))?;

        self.peer_addr = Some(addr);

        Ok(n)
    }
}

impl UdpConnection<ClientSide> {
    pub async fn connect(&mut self, addr: SocketAddr) -> Result<(), UdpError> {
        self.socket.connect(addr)
            .await
            .map_err(|e| UdpError::Std(format!("connect error: {e}")))?;

        Ok(())
    }

    pub async fn send(&mut self) -> Result<usize, UdpError> {
        let duration: Duration = self.config.common.send_timeout_secs;
        let buf: &mut BytesMut = &mut self.write_buf;

        let n: usize = timeout(duration, self.socket.send(buf))
            .await
            .map_err(|e| UdpError::Timeout(format!("send, elapsed: {e}")))?
            .map_err(|e| UdpError::Std(format!("send error: {e}")))?;

        Ok(n)
    }

    pub async fn recv(&mut self) -> Result<usize, UdpError> {
        let duration: Duration = self.config.common.recv_timeout_secs;
        let buf: &mut BytesMut = &mut self.read_buf;

        let n: usize = timeout(duration, self.socket.recv(buf))
            .await
            .map_err(|e| UdpError::Timeout(format!("recv from, elapsed: {e}")))?
            .map_err(|e| UdpError::Std(format!("recv from error: {e}")))?;

        Ok(n)
    }
}