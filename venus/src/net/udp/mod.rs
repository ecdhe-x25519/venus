pub mod acceptor;
pub mod connector;

#[cfg(test)]
mod udp_test {
    use super::acceptor::*;
    use super::connector::*;

    #[tokio::test]
    async fn test_udp() {
        let common = UdpCommonConfig::new(
            "127.0.0.1:1234",
            None,
            false,
            1024 * 16,
            60,
            60,
        ).unwrap();

        let config = UdpServerConfig::new(
            common,
            false,
            None,
        ).unwrap();

        let mut server: UdpServer = UdpServer::bind(config).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let _server_handle = tokio::spawn(async move {
            let conn = server.handle();
            let data: &[u8; 7] = b"pidoras";
            conn.write_frame(data);
            loop {
                let (d, sa) = conn.recv().await.unwrap();

                let ascii = std::str::from_utf8(&d).unwrap();
                println!("{}", ascii);
                
                conn.send(Some(sa)).await.unwrap();
            }
        });

        let common = UdpCommonConfig::new(
            "0.0.0.0:0",
            None,
            false,
            1024 * 16,
            60,
            60,
        ).unwrap();

        let config = UdpClientConfig::new(
            common,
            "127.0.0.1:1234",
        ).unwrap();

        let mut client: UdpClient = UdpClient::connect(config).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let _client_handle = tokio::spawn( async move {
            let conn = client.handle();
            let data: &[u8; 7] = b"pidoras";
            conn.write_frame(data);
            loop {

                conn.send().await.unwrap();
    
                let d = conn.recv().await.unwrap();
                let ascii = std::str::from_utf8(&d).unwrap();
    
                println!("{}", ascii);
            }
        });

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}