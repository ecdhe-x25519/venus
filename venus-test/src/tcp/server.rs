use venus::net::tcp::acceptor::*;

async fn start_tcp_server() {
    let common = TcpCommonConfig::new(
        1024 * 16,
        5 * 60,
        false,
        500,
        255,
    ).unwrap();

    let config = TcpServerConfig::new(
        common,
        100,
        "0.0.0.0:1234",
        false,
    ).unwrap();

    let server: TcpServer = TcpServer::bind(config).await.unwrap();
}