pub mod acceptor;
pub mod connector;

#[cfg(test)]
mod tcp_test {
    use super::acceptor::*;
    use super::connector::*;

    #[tokio::test]
    async fn test_tcp() {
        let common = TcpCommonConfig::new(
            1024 * 16,
            5,
            true,
            true,
            500,
            255,
        ).unwrap();

        let config = TcpServerConfig::new(
            common,
            101,
            "0.0.0.0:1234",
        ).unwrap();

        let server: TcpServer = TcpServer::bind(config).await.unwrap();

        let _server_handle = tokio::spawn(async move {
            let mut conn = server.handle().await.unwrap();
            let data: &[u8; 7] = b"pidoras";
            loop {
                let z = conn.read_frame().await.unwrap();

                conn.write_frame(data).await.unwrap();

                let ascii = std::str::from_utf8(&z).unwrap();

                println!("{}", ascii);
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        for _ in 0..100 {
            tokio::spawn(async move {
                let common = TcpCommonConfig::new(
                    1024 * 16,
                    5 * 60,
                    true,
                    true,
                    500,
                    255,
                ).unwrap();

                let config = TcpClientConfig::new(
                    common,
                    "127.0.0.1:1234",
                    5,
                ).unwrap();

                let client: TcpClient = TcpClient::connect(config).await.unwrap();

                let mut conn = client.handle().await.unwrap();

                let data: &[u8; 7] = b"pidoras";
                loop {
                    conn.write_frame(data).await.unwrap();

                    let z = conn.read_frame().await.unwrap();

                    let ascii = std::str::from_utf8(&z).unwrap();

                    println!("{}", ascii);
                }
            });
        };

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}