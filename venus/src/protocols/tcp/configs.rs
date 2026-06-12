use std::net::SocketAddr;
use std::sync::Arc;

use tokio::time::Duration;

use crate::error::IoError;

#[derive(Debug)]
pub struct TcpServerConfig {
    pub(crate) common: Arc<TcpCommonConfig>,
    pub(crate) max_conns: usize,
    pub(crate) listen_addr: SocketAddr,
}

impl TcpServerConfig {
    pub fn new(
        common: Arc<TcpCommonConfig>,
        max_conns: usize,
        listen_addr: &str,
    ) -> Result<Arc<Self>, IoError> {
        let listen_addr: SocketAddr = listen_addr.parse()
            .map_err(|e| IoError(format!("incorrect addr: {e}")))?;

        Ok(Arc::new(Self {
            common,
            max_conns,
            listen_addr,
        }))
    }
}

#[derive(Debug)]
pub struct TcpClientConfig {
    pub(crate) common: Arc<TcpCommonConfig>,
    pub(crate) dest_addr: SocketAddr,
    pub(crate) conn_timeout_secs: Duration,
}

impl TcpClientConfig {
    pub fn new(
        common: Arc<TcpCommonConfig>,
        dest_addr: &str,
        conn_timeout_secs: u16,
    ) -> Result<Arc<Self>, IoError> {
        let dest_addr: SocketAddr = dest_addr.parse()
            .map_err(|e| IoError(format!("incorrect addr: {e}")))?;

        let conn_timeout_secs: Duration = Duration::from_secs(conn_timeout_secs as u64);

        Ok(Arc::new(Self {
            common,
            dest_addr,
            conn_timeout_secs,
        }))
    }
}

#[derive(Debug)]
pub struct TcpCommonConfig {
    pub(crate) buffers_capacity: usize,
    pub(crate) idle_timeout_secs: Duration,
    pub(crate) no_delay: bool,
    pub(crate) instant_ack: bool,
    pub(crate) max_fragment_size: u16,
    pub(crate) sending_interval_nanosecs: Duration,
}

impl TcpCommonConfig {
    pub fn new(
        buffers_capacity: usize,
        idle_timeout_secs: u16,
        no_delay: bool,
        instant_ack: bool,
        max_fragment_size: u16,
        sending_interval_nanosecs: u16,
    ) -> Result<Arc<Self>, IoError> {
        let idle_timeout_secs: Duration = Duration::from_secs(idle_timeout_secs as u64);
        let sending_interval_nanosecs: Duration = Duration::from_nanos(sending_interval_nanosecs as u64);

        Ok(Arc::new(Self {
            buffers_capacity,
            idle_timeout_secs,
            no_delay,
            instant_ack,
            max_fragment_size,
            sending_interval_nanosecs,
        }))
    }
}