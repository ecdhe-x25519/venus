pub struct TcpServerConfig {
    pub common: TcpCommonConfig,
    pub max_conns: usize,
    pub listen_addr: String,
    pub instant_ack: bool,
}

pub struct TcpClientConfig {
    pub common: TcpCommonConfig,
    pub dest_addr: String,
    pub conn_timeout_secs: String,
}

pub struct TcpCommonConfig {
    pub rx_tx_bufs_capacity: usize,
    pub idle_timeout_secs: usize,
    pub no_delay: bool,
    pub application_proto: TcpAppProto,
}

pub enum TcpAppProto {
    Raw,
    Tls,
    Http2,
    Http3,
}