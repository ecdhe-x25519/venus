use std::net::{IpAddr, Ipv4Addr};

use crate::error::IoError;

#[derive(Debug)]
pub struct IfaceConfig {
    pub(crate) name: String,
    pub(crate) mtu: u16,
    pub(crate) addr: IpAddr,
    pub(crate) netmask: Ipv4Addr,
    pub(crate) destination: Option<IpAddr>,
    pub(crate) buf_capacity: usize,
    pub(crate) fd: Option<i32>,
    pub(crate) offload: bool,
}

impl IfaceConfig {
    pub fn new(
        name: &str,
        mtu: u16,
        addr: &str,
        netmask: &str,
        destination: Option<&str>,
        buf_capacity: usize,
        async_mode: bool,
        unix: bool,
        fd: Option<i32>,
        offload: bool,
    ) -> Result<Self, IoError> {
        if async_mode && unix {
            return Err(IoError("async mode not available on unix platform".to_string()))
        };

        if unix && fd.is_none() {
            return Err(IoError("fd should be set on unix platform".to_string()))
        };

        let addr: IpAddr = addr.parse()
            .map_err(|e| IoError(format!("invalid addr: {e}")))?;

        let netmask: Ipv4Addr = netmask.parse()
            .map_err(|e| IoError(format!("invalid netmask addr: {e}")))?;

        let destination: Option<IpAddr> = match destination {
            Some(dest) => Some(dest.parse().map_err(|e| IoError(format!("invalid destination addr: {e}")))?),
            None => None,
        };

        Ok(Self {
            name: name.to_owned(),
            mtu,
            addr,
            netmask,
            destination,
            buf_capacity,
            fd,
            offload,
        })
    }
}