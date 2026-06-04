use std::sync::Arc;

use crate::protocols::tcp::configs::*;
use crate::protocols::tcp::connection::*;

use crate::error::Error;

use tokio::net::TcpListener;
use tokio::sync::Semaphore;

pub struct TcpServer {
    pub config: TcpServerConfig,
    pub listener: TcpListener,
}

impl TcpServer {
    pub async fn bind(config: TcpServerConfig) -> Result<Self, Error> {
        let listener = TcpListener::bind(&config.listen_addr)
            .await
            .map_err(|e| Error::Std(format!("bind error: {e}")))?;

        Ok(Self {
            config,
            listener
        })
    }

    pub async fn handle_incoming(&self) -> Result<(), Error> {
        let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(self.config.max_conns));

        loop {
            semaphore.acquire().await.map_err(|e| Error::Std(format!("semaphore error: {e}")))?;

            let (stream, peer_addr) = self.listener.accept()
                .await
                .map_err(|e| Error::Std(format!("connection error: {e}")))?;

            let conn: TcpConnection = TcpConnection::new(stream, peer_addr.to_string(), &self.config.common)
                .await
                .map_err(|e| Error::Std(format!("new connection error: {e}")))?;
        }
    }
}