use crate::protocols::tcp::configs::*;
use crate::protocols::tcp::connection::*;

use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::error::TcpError;

pub struct TcpClient {
    config: TcpClientConfig,
    conn: TcpStream,
}

impl TcpClient {
    pub async fn connect(config: TcpClientConfig) -> Result<Self, TcpError> {
        let conn: TcpStream = timeout(config.conn_timeout_secs, TcpStream::connect(&config.dest_addr))
            .await
            .map_err(|e| TcpError::Timeout(format!("connection timeout, elapsed: {e}")))?
            .map_err(|e| TcpError::Std(format!("connection error: {e}")))?;

        Ok(Self {
            config,
            conn,
        })
    }

    pub async fn handle(self) -> Result<TcpConnection, TcpError> {
        let conn: TcpConnection = TcpConnection::new(
            self.conn,
            self.config.common
        ).await?;

        Ok(conn)
    }
}