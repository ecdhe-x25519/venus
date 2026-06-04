use std::net::SocketAddr;

use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use crate::error::Error;
use super::configs::{TcpCommonConfig, TcpAppProto};

use bytes::*;

pub struct TcpConnection {
    pub peer_addr: String,
    pub read_buf: BytesMut,
    pub write_buf: BytesMut,
    pub read_in: OwnedReadHalf,
    pub write_out: OwnedWriteHalf,
}

impl TcpConnection {
    pub async fn new(stream: TcpStream, peer_addr: String, cfg: &TcpCommonConfig) -> Result<Self, Error> {
        let (read_in, write_out) = stream.into_split();

        Ok(Self {
            peer_addr,
            read_buf: BytesMut::with_capacity(cfg.rx_tx_bufs_capacity),
            write_buf: BytesMut::with_capacity(cfg.rx_tx_bufs_capacity),
            read_in,
            write_out,
        })
    }

    pub async fn handle(cfg: &TcpCommonConfig) -> Result<(), Error> {
        match cfg.application_proto {
            TcpAppProto::Raw => handle_raw(),
            TcpAppProto::Tls => handle_tls(),
            TcpAppProto::Http2 => handle_http2(),
            TcpAppProto::Http3 => handle_http3(),
        }
    }
}