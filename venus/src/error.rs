#[derive(Debug)]
pub enum Error {
    Std(String),
    Tls(String),
    Http(String),
    Timeout(String),
    ConnectionClosed,
}