use crate::protocols::tcp::configs::*;
use crate::protocols::tcp::connection::*;

use crate::error::Error;

use tokio::net::TcpStream;

pub struct TcpClient {
    pub config: TcpClientConfig,
    pub conn: TcpStream,
}

impl TcpClient {
    pub async fn bind(config: TcpClientConfig) -> Result<Self, Error> {
        let conn = TcpStream::connect(&config.dest_addr)
            .await
            .map_err(|e| Error::Std(format!("bind error: {e}")))?;

        Ok(Self {
            config,
            conn,
        })
    }

    pub async fn handle(&self) -> Result<(), Error> {
        loop {
            let conn: TcpConnection = TcpConnection::new(self.conn, self.config.dest_addr, &self.config.common)
                .await
                .map_err(|e| Error::Std(format!("new connection error: {e}")))?;
        }
    }
}