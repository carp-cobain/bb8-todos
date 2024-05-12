use crate::signer::{Signer, Verifier};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

/// Configuration settings
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub db_url: String,
    pub db_max_pool_size: u32,
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
        // http
        let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".into());
        let listen_addr = format!("0.0.0.0:{}", port);

        // db connection
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        // db pool
        let mut db_max_pool_size = num_cpus::get() as u32;
        if let Ok(s) = env::var("DATABASE_MAX_POOL_SIZE") {
            db_max_pool_size = s
                .parse()
                .expect("DATABASE_MAX_POOL_SIZE could not be parsed")
        }

        // verify signing key env vars are set and valid
        let _: Signer = Default::default();
        let _: Verifier = Default::default();

        Self {
            listen_addr,
            db_url,
            db_max_pool_size,
        }
    }

    pub async fn tcp_listener(&self) -> TcpListener {
        let addr: SocketAddr = self
            .listen_addr
            .parse()
            .expect("Failed to parse listen address");

        TcpListener::bind(addr)
            .await
            .expect("failed to bind tcp listener")
    }
}
