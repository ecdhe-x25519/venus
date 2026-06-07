use std::fmt;

pub struct IoError(pub(crate) String);

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error: {}", self.0)
    }
}

pub enum TcpError {
    Std(String),
    Timeout(String),
    ConnectionClosed,
}

impl fmt::Display for TcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Std(e) => write!(f, "TCP standard error: {e}"),
            Self::Timeout(e) => write!(f, "TCP connection timeout: {e}"),
            Self::ConnectionClosed => write!(f, "TCP connection closed"),
        }
    }
}

pub enum UdpError {
    Std(String),
    Timeout(String),
}

impl fmt::Display for UdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Std(e) => write!(f, "UDP standard error: {e}"),
            Self::Timeout(e) => write!(f, "UDP connection timeout. Elapsed time: {e}"),
        }
    }
}