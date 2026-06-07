use std::sync::Arc;

use crate::protocols::tcp::configs::*;
use crate::protocols::tcp::connection::*;

use crate::error::TcpError;

use tokio::net::TcpListener;
use tokio::sync::Semaphore;

pub struct TcpServer {
    config: TcpServerConfig,
    listener: TcpListener,
    semaphore: Arc<Semaphore>,
}

impl TcpServer {
    pub async fn bind(config: TcpServerConfig) -> Result<Self, TcpError> {
        let listener: TcpListener = TcpListener::bind(&config.listen_addr)
            .await
            .map_err(|e| TcpError::Std(format!("bind error: {e}")))?;

        let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(config.max_conns));

        Ok(Self {
            config,
            listener,
            semaphore,
        })
    }

    pub async fn handle_incoming(&self) -> Result<TcpConnection, TcpError> {
        self.semaphore.acquire()
            .await
            .map_err(|e| TcpError::Std(format!("semaphore acquire error: {e}")))?;

        let stream = self.listener.accept()
            .await
            .map_err(|e| TcpError::Std(format!("connection error: {e}")))?.0;

        let conn: TcpConnection = TcpConnection::new(
            stream,
            self.config.common.clone()
        ).await?;

        Ok(conn)
    }
}