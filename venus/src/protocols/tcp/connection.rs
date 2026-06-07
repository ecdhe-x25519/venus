use std::mem::take;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use tokio::time::{Duration, timeout, interval};

use super::configs::TcpCommonConfig;

use crate::error::*;

use bytes::*;

pub struct TcpConnection {
    config: Arc<TcpCommonConfig>,
    peer_addr: SocketAddr,
    read_buf: BytesMut,
    write_buf: BytesMut,
    read_in: OwnedReadHalf,
    write_out: OwnedWriteHalf,
}

impl TcpConnection {
    pub async fn new(stream: TcpStream, config: Arc<TcpCommonConfig>) -> Result<Self, TcpError> {
        let peer: &SocketAddr = &stream.peer_addr()
            .map_err(|e| TcpError::Std(format!("get socket addr error: {e}")))?;

        let (read_in, write_out) = stream.into_split();

        let capacity: usize = config.buffers_capacity;

        Ok(Self {
            config,
            peer_addr: *peer,
            read_buf: BytesMut::with_capacity(capacity),
            write_buf: BytesMut::with_capacity(capacity),
            read_in,
            write_out,
        })
    }

    pub async fn read_frame(&mut self) -> Result<usize, TcpError> {
        let duration: Duration = self.config.idle_timeout_secs;

        let n: Result<usize, TcpError> = timeout(duration, self.read_with_timeout())
            .await
            .map_err(|e| TcpError::Timeout(format!("idle timeout, elapsed: {e}")))?;

        match n {
            Ok(n) => Ok(n),
            Err(e) => return Err(TcpError::Std(format!("connection error: {e}")))
        }
    }

    pub async fn write_frame(&mut self, data: &[u8]) -> Result<(), TcpError> {
        self.write_buf.put_slice(data);
        self.send().await?;

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), TcpError> {
        let residual_data: BytesMut = take(&mut self.write_buf);
        self.write_frame(&residual_data).await?;

        self.write_out.shutdown()
            .await
            .map_err(|e| TcpError::Std(format!("shutdown error: {e}")))?;

        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }

    pub async fn force_close(&mut self) -> Result<(), TcpError> {
        self.write_out.shutdown()
            .await
            .map_err(|e| TcpError::Std(format!("shutdown error: {e}")))?;

        Ok(())
    }

    async fn send(&mut self) -> Result<(), TcpError> {
        if self.config.no_delay {
            let max: usize = self.config.max_fragment_size as usize;
            let mut interval = interval(self.config.sending_interval_nanosecs);

            interval.tick().await;

            while !self.write_buf.is_empty() {
                interval.tick().await;
                
                let to_send: usize = self.write_buf.len().min(max);
                let data: &[u8] = &self.write_buf[..to_send];
                
                self.write_out.write_all(data).await
                    .map_err(|e| TcpError::Std(format!("write error: {e}")))?;
                
                self.write_out.flush().await
                    .map_err(|e| TcpError::Std(format!("flush error: {e}")))?;
                
                self.write_buf.advance(to_send);
            }
        } else {
            self.write_out.write_all(&self.write_buf)
                .await
                .map_err(|e| TcpError::Std(format!("write error: {e}")))?;
            
            self.write_buf.clear();
        }

        Ok(())
    }

    async fn read_with_timeout(&mut self) -> Result<usize, TcpError> {
        let n: usize = self.read_in.read_buf(&mut self.read_buf)
            .await
            .map_err(|e| TcpError::Std(format!("read error: {e}")))?;

        Ok(n)
    }
}