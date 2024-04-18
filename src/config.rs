use std::{env, io, net::SocketAddr};
use tokio::net::{TcpListener, TcpSocket};

/// Configuration settings
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub db_url: String,
}

/// Default for config just calls basic constructor
impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".into());
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let listen_addr = format!("0.0.0.0:{}", port);
        Self {
            listen_addr,
            db_url,
        }
    }

    pub fn tcp_listener(&self) -> TcpListener {
        let addr: SocketAddr = self
            .listen_addr
            .parse()
            .expect("Failed to parse listen address");

        reuse_listener(addr).expect("Failed calling reuse_listener")
    }
}

fn reuse_listener(addr: SocketAddr) -> io::Result<TcpListener> {
    let socket = match addr {
        SocketAddr::V4(_) => TcpSocket::new_v4()?,
        SocketAddr::V6(_) => TcpSocket::new_v6()?,
    };
    #[cfg(unix)]
    {
        if let Err(e) = socket.set_reuseport(true) {
            tracing::warn!("error setting SO_REUSEPORT: {}", e);
        }
    }
    if let Err(e) = socket.set_reuseaddr(true) {
        tracing::warn!("error calling set_reuseaddr: {}", e);
    }
    if let Err(e) = socket.set_nodelay(true) {
        tracing::warn!("error calling set_nodelay: {}", e);
    }
    socket.bind(addr)?;
    socket.listen(1024)
}
