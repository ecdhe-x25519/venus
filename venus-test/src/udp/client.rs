use venus::net::udp::connector::*;

async fn start_udp_client() {
    let common = UdpCommonConfig::new(
        "127.0.0.1:1234",
        None,
        1024 * 16,
        false,
        60 * 5,
        60 * 5,
    ).unwrap();

    let config = UdpClientConfig::new(
        common,
        "127.0.0.1:1234",
    ).unwrap();

    let client: UdpClient = UdpClient::new(config).await.unwrap();
}