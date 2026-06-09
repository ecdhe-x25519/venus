use venus::net::tcp::connector::*;

async fn start_tcp_client() {
    let common = TcpCommonConfig::new(
        1024 * 16,
        5 * 60,
        false,
        500,
        255,
    ).unwrap();

    let config = TcpClientConfig::new(
        common,
        "127.0.0.1:1234",
        5,
    ).unwrap();

    let server: TcpClient = TcpClient::connect(config).await.unwrap();
}