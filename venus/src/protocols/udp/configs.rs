use std::net::SocketAddr;
use std::sync::Arc;

use tokio::time::Duration;

use crate::error::IoError;

pub trait Side {
    type Config;
    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig>;
    fn is_client() -> bool;
}

pub struct ClientSide;

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

impl Side for ClientSide {
    type Config = UdpClientConfig;

    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig> {
        config.common.clone()
    }

    fn is_client() -> bool {
        true
    }
}

pub struct ServerSide;

#[derive(Debug)]
pub struct UdpServerConfig {
    pub(crate) common: Arc<UdpCommonConfig>,
    pub(crate) enable_connections: bool,
    pub(crate) pre_init_conns: bool,
    pub(crate) max_conns: Option<usize>,
}

impl UdpServerConfig {
    pub fn new(
        common: Arc<UdpCommonConfig>,
        enable_connections: bool,
        pre_init_conns: bool,
        max_conns: Option<usize>,
    ) -> Result<Arc<Self>, IoError> {
        if max_conns.is_some() && !enable_connections {
            return Err(IoError("max_conns requires connection_mode = true".to_string()));
        };

        if max_conns.is_some() && max_conns.unwrap() <= 0 && enable_connections {
            return Err(IoError("max_conns cant be <= 0".to_string()));
        };

        Ok(Arc::new(Self {
            common,
            enable_connections,
            pre_init_conns,
            max_conns,
        }))
    }
}

impl Side for ServerSide {
    type Config = UdpServerConfig;

    fn common(config: Arc<Self::Config>) -> Arc<UdpCommonConfig> {
        config.common.clone()
    }

    fn is_client() -> bool {
        false
    }
}

#[derive(Debug)]
pub struct UdpCommonConfig {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) device: Option<Vec<u8>>,
    pub(crate) enable_broadcast: bool,
    pub(crate) buffers_capacity: usize,
    pub(crate) recv_timeout_secs: Duration,
    pub(crate) send_timeout_secs: Duration,
}

impl UdpCommonConfig {
    pub fn new(
        bind_addr: &str,
        device: Option<Vec<u8>>,
        buffers_capacity: usize,
        enable_broadcast: bool,
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
            buffers_capacity,
            enable_broadcast,
            recv_timeout_secs,
            send_timeout_secs,
        }))
    }
}