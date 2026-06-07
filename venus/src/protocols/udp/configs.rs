use std::net::SocketAddr;

use tokio::time::Duration;

use crate::error::IoError;

#[derive(Debug)]
pub struct UdpCommonConfig {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) buffers_capacity: usize,
    pub(crate) connection_mode: bool,
    pub(crate) max_conns: usize,
    pub(crate) recv_timeout_secs: Duration,
    pub(crate) send_timeout_secs: Duration,
}

impl UdpCommonConfig {
    pub fn new(
        bind_addr: &str,
        buffers_capacity: usize,
        connection_mode: bool,
        max_conns: usize,
        recv_timeout_secs: u16,
        send_timeout_secs: u16,
    ) -> Result<Self, IoError> {
        let bind_addr: SocketAddr = bind_addr.parse()
            .map_err(|e| IoError(format!("incorrect addr: {e}")))?;

        let recv_timeout_secs: Duration = Duration::from_secs(recv_timeout_secs as u64);
        let send_timeout_secs: Duration = Duration::from_secs(send_timeout_secs as u64);

        Ok(Self {
            bind_addr,
            buffers_capacity,
            connection_mode,
            max_conns,
            recv_timeout_secs,
            send_timeout_secs,
        })
    }
}