use venus::net::udp::acceptor::*;

async fn start_udp_server() {
    let common = UdpCommonConfig::new(
        "127.0.0.1:1234",
        None,
        1024 * 16,
        false,
        60 * 5,
        60 * 5,
    ).unwrap();

    let config = UdpServerConfig::new(
        common,
        false,
        false,
        None,
    ).unwrap();

    let server: UdpServer = UdpServer::new(config).await.unwrap();
}