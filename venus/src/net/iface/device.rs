use tun_rs::{SyncDevice, AsyncDevice};
use tun_rs::DeviceBuilder;

use crate::error::IoError;

pub use super::configs::IfaceConfig;

pub trait DeviceMode {
    type Device: DeviceIO;
    
    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError>;
}

pub trait DeviceIO: Send + Sync {
    fn recv_io(&mut self, buf: &mut [u8]) -> impl std::future::Future<Output = Result<usize, IoError>> + Send;
    fn send_io(&mut self, buf: &[u8]) -> impl std::future::Future<Output = Result<usize, IoError>> + Send;
}

pub struct Device<M: DeviceMode> {
    _config: IfaceConfig,
    inner: M::Device,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
}

impl<M: DeviceMode> Device<M> {
    pub fn new(config: IfaceConfig) -> Result<Self, IoError> {
        let inner = M::build(&config)?;
        Ok(Self {
            read_buf: Vec::with_capacity(config.buf_capacity),
            write_buf: Vec::with_capacity(config.buf_capacity),
            _config: config,
            inner,
        })
    }

    pub async fn recv(&mut self) -> Result<usize, IoError> {
        let n = self.inner.recv_io(&mut self.read_buf).await?;

        Ok(n)
    }

    pub async fn send(&mut self) -> Result<usize, IoError> {
        let n = self.inner.send_io(&mut self.write_buf).await?;

        Ok(n)
    }
}

pub struct SyncMode;

impl DeviceMode for SyncMode {
    type Device = SyncDevice;

    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        DeviceBuilder::new()
            .name(&config.name)
            .ipv4(config.addr, config.netmask, config.destination)
            .mtu(config.mtu)
            .offload(config.offload)
            .enable(true)
            .build_sync()
            .map_err(|e| IoError(format!("build error: {e}")))
    }
}

impl DeviceIO for SyncDevice {
    async fn recv_io(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let n = self.recv(buf)
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }

    async fn send_io(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let n = self.send(buf)
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }
}

pub struct AsyncMode;

impl DeviceMode for AsyncMode {
    type Device = AsyncDevice;

    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        DeviceBuilder::new()
            .name(&config.name)
            .ipv4(config.addr, config.netmask, config.destination)
            .mtu(config.mtu)
            .offload(config.offload)
            .enable(true)
            .build_async()
            .map_err(|e| IoError(format!("build error: {e}")))
    }
}

impl DeviceIO for AsyncDevice {
    async fn recv_io(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let n = self.recv(buf)
            .await
            .map_err(|e| IoError(format!("recv error: {e}")))?;

        Ok(n)
    }

    async fn send_io(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let n = self.send(buf)
            .await
            .map_err(|e| IoError(format!("send error: {e}")))?;

        Ok(n)
    }
}

#[cfg(unix)]
pub struct UnixMode;

#[cfg(unix)]
impl DeviceMode for UnixMode {
    type Device = SyncDevice;
    
    fn build(config: &IfaceConfig) -> Result<Self::Device, IoError> {
        let fd = config.fd.ok_or_else(|| IoError("unix fd required".to_string()))?;
        unsafe {
            SyncDevice::from_fd(fd).map_err(|e| IoError(format!("build unix device error: {e}")))
        }
    }
}