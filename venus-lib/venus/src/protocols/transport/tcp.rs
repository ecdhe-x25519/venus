use std::sync::Arc;

use tokio::io::{self, copy_bidirectional};
use tokio::sync::Semaphore;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};
use tokio::select;

use bytes::BytesMut;

use crate::error::Error;

pub struct ServerConfig {
    pub listen_addr: String,
    pub max_conns: usize,
    pub timeout: Duration,
}

pub async fn start_listener(cfg: ServerConfig) -> Result<(), Error> {
    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(cfg.max_conns));

    let listener: TcpListener = TcpListener::bind(cfg.listen_addr)
        .await
        .map_err(|_| Error::ListeningError)?;

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let (stream, addr) = listener.accept()
            .await
            .map_err(|_| Error::ConnectionError)?;

        println!("TCP connection from: {}", addr);

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = handle_connection(stream, &cfg.timeout).await {
                eprintln!("Proxy error");
            }
        });
    }

    Ok(())
}

pub async fn handle_connection(client: TcpStream, timeout: &Duration) -> Result<(), Error> {
    let server = timeout(Duration::from_secs(5), TcpStream::connect(&target_addr))
        .await
        .map_err(|_| "connection timeout")?
        .map_err(|e| e)?;
    
    proxy_with_idle_timeout(client, server, timeout).await?;

    Ok(())
}

pub async fn handle_proxy(
    mut client: TcpStream,
    mut server: TcpStream,
    timeout: Duration
) -> Result<(), Error> {
    let mut client_buf: BytesMut = BytesMut::with_capacity(8192);
    let mut server_buf: BytesMut = BytesMut::with_capacity(8192);
    let mut last_activity = tokio::time::Instant::now();

    loop {
        select! {
            res = client.read(&mut client_buf) => {
                let n = res?;
                if n == 0 { break; }
                server.write_all(&client_buf[..n]).await?;
                last_activity = tokio::time::Instant::now();
            }
            res = server.read(&mut server_buf) => {
                let n = res?;
                if n == 0 { break; }
                client.write_all(&server_buf[..n]).await?;
                last_activity = tokio::time::Instant::now();
            }
            _ = tokio::time::sleep_until(last_activity + timeout) => {
                eprintln!("Idle timeout reached, closing connection");
                return Err("idle timeout".into());
            }
        };
    }

    Ok(())
}