use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::time::{Duration, timeout};

use super::configs::*;

use crate::error::UdpError;

use bytes::*;

pub struct UdpConnection<S: UdpSide> {
    pub(crate) config: Arc<S::Config>,
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) peer_addr: Option<SocketAddr>,
    pub(crate) read_buf: BytesMut,
    pub(crate) write_buf: BytesMut,
}

impl<S: UdpSide> UdpConnection<S> {
    pub(crate) async fn new(config: Arc<S::Config>, peer_addr: Option<SocketAddr>) -> Result<Self, UdpError> {
        let common = S::common(config.clone());

        let socket: UdpSocket = UdpSocket::bind(common.bind_addr)
            .await
            .map_err(|e| UdpError::Std(format!("bind error: {e}")))?;

        socket.bind_device(common.device.as_deref())
            .map_err(|e| UdpError::Std(format!("bind device error: {e}")))?;

        socket.set_broadcast(common.broadcast)
            .map_err(|e| UdpError::Std(format!("set broadcast error: {e}")))?;

        if peer_addr.is_some() {
            socket.connect(peer_addr.unwrap())
                .await
                .map_err(|e| UdpError::Std(format!("connect error: {e}")))?;
        }

        let capacity: usize = common.buffers_capacity;

        Ok(Self {
            config,
            socket: Arc::new(socket),
            peer_addr: peer_addr,
            read_buf: BytesMut::with_capacity(capacity),
            write_buf: BytesMut::with_capacity(capacity),
        })
    }

    fn read_frame(&mut self, len: usize) -> BytesMut {
        self.read_buf.split_to(len)
    }

    pub fn write_frame(&mut self, data: &[u8]) {
        self.write_buf.clear();
        self.write_buf.put(data)
    }
}

impl UdpConnection<UdpServerSide> {
    pub async fn recv(&mut self) -> Result<(BytesMut, SocketAddr), UdpError> {
        let duration: Duration = self.config.common.recv_timeout_secs;
        let buf: &mut BytesMut = &mut self.read_buf;

        let (n, addr) = timeout(duration, self.socket.recv_buf_from(buf))
            .await
            .map_err(|_| UdpError::Timeout(format!("recv timeout")))?
            .map_err(|e| UdpError::Std(format!("recv error: {e}")))?;

        n_check(n)?;

        let data = self.read_frame(n);

        println!("{}", self.read_buf.capacity());

        Ok((data, addr))
    }

    pub async fn accept(&mut self) -> Result<(BytesMut, UdpConnection<UdpServerSide>), UdpError> {
        let duration: Duration = self.config.common.recv_timeout_secs;
            let buf: &mut BytesMut = &mut self.read_buf;

            let (n, addr) = timeout(duration, self.socket.recv_from(buf))
                .await
                .map_err(|_| UdpError::Timeout(format!("recv_from timeout")))?
                .map_err(|e| UdpError::Std(format!("recv_from error: {e}")))?;

            n_check(n)?;

            let conn: UdpConnection<UdpServerSide> = UdpConnection::new(self.config.clone(), Some(addr)).await?;

            let data = self.read_frame(n);

            Ok((data, conn))
    }

    pub async fn send(&mut self, peer_addr: Option<SocketAddr>) -> Result<(), UdpError> {
        let duration: Duration = self.config.common.send_timeout_secs;
        let buf: &mut BytesMut = &mut self.write_buf;

        let mut addr = self.peer_addr;
        if self.peer_addr.is_none() && peer_addr.is_some() {
            addr = peer_addr
        }

        let n: usize = timeout(duration, self.socket.send_to(buf, addr.unwrap()))
            .await
            .map_err(|_| UdpError::Timeout(format!("send_to timeout")))?
            .map_err(|e| UdpError::Std(format!("send_to error: {e}")))?;

        n_check(n)?;

        Ok(())
    }
}

impl UdpConnection<UdpClientSide> {
    pub async fn recv(&mut self) -> Result<BytesMut, UdpError> {
        let duration: Duration = self.config.common.recv_timeout_secs;
        let buf: &mut BytesMut = &mut self.read_buf;

        let n: usize = timeout(duration, self.socket.recv_buf(buf))
            .await
            .map_err(|_| UdpError::Timeout(format!("recv timeout")))?
            .map_err(|e| UdpError::Std(format!("recv error: {e}")))?;

        n_check(n)?;

        let data = self.read_frame(n);

        Ok(data)
    }

    pub async fn send(&mut self) -> Result<(), UdpError> {
        let duration: Duration = self.config.common.send_timeout_secs;
        let buf: &mut BytesMut = &mut self.write_buf;

        let n: usize = timeout(duration, self.socket.send(buf))
            .await
            .map_err(|_| UdpError::Timeout(format!("send timeout")))?
            .map_err(|e| UdpError::Std(format!("send error: {e}")))?;

        n_check(n)?;

        Ok(())
    }
}

pub(crate) fn n_check(n: usize) -> Result<(), UdpError> {
    if n == 0 {
        return Err(UdpError::Timeout("received no bytes".to_string()))
    }

    Ok(())
}