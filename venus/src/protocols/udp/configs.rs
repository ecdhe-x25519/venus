use std::net::SocketAddr;
use std::sync::Arc;

use tokio::time::Duration;

use crate::error::IoError;

pub trait UdpSide: Send + Sync {
    type Config: Send + Sync;
    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig>;
}

pub struct UdpServerSide;
impl UdpSide for UdpServerSide {
    type Config = UdpServerConfig;
    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig> {
        config.common.clone()
    }
}

pub struct UdpClientSide;
impl UdpSide for UdpClientSide {
    type Config = UdpClientConfig;
    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig> {
        config.common.clone()
    }
}


#[derive(Debug)]
pub struct UdpServerConfig {
    pub(crate) common: Arc<UdpCommonConfig>,
    pub(crate) connection_mode: bool,
    pub(crate) max_conns: Option<usize>,
}

impl UdpServerConfig {
    pub fn new(
        common: Arc<UdpCommonConfig>,
        connection_mode: bool,
        max_conns: Option<usize>,
    ) -> Result<Arc<Self>, IoError> {
        if max_conns.is_some() && !connection_mode {
            return Err(IoError("max_conns requires connection mode".to_string()))
        }

        Ok(Arc::new(Self {
            common,
            connection_mode,
            max_conns,
        }))
    }
}

#[derive(Debug)]
pub struct UdpClientConfig {
    pub(crate) common: Arc<UdpCommonConfig>,
    pub(crate) remote_addr: SocketAddr,
}

impl UdpClientConfig {
    pub fn new(
        common: Arc<UdpCommonConfig>,
        remote_addr: &str,
    ) -> Result<Arc<Self>, IoError> {
        let remote_addr: SocketAddr = remote_addr.parse()
            .map_err(|e| IoError(format!("invalid remote addr: {e}")))?;

        Ok(Arc::new(Self {
            common,
            remote_addr,
        }))
    }
}

#[derive(Debug)]
pub struct UdpCommonConfig {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) device: Option<Vec<u8>>,
    pub(crate) broadcast: bool,
    pub(crate) buffers_capacity: usize,
    pub(crate) recv_timeout_secs: Duration,
    pub(crate) send_timeout_secs: Duration,
}

impl UdpCommonConfig {
    pub fn new(
        bind_addr: &str,
        device: Option<Vec<u8>>,
        broadcast: bool,
        buffers_capacity: usize,
        recv_timeout_secs: u16,
        send_timeout_secs: u16,
    ) -> Result<Arc<Self>, IoError> {
        let bind_addr: SocketAddr = bind_addr.parse()
            .map_err(|e| IoError(format!("invalid bind addr: {e}")))?;

        let recv_timeout_secs: Duration = Duration::from_secs(recv_timeout_secs as u64);
        let send_timeout_secs: Duration = Duration::from_secs(send_timeout_secs as u64);

        Ok(Arc::new(Self {
            bind_addr,
            device,
            broadcast,
            buffers_capacity,
            recv_timeout_secs,
            send_timeout_secs,
        }))
    }
}