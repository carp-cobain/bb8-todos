use crate::{db::sql, Error, Result};
use async_trait::async_trait;
use bb8::{CustomizeConnection, Pool, RunError};
use std::error::Error as StdError;
use std::str::FromStr;
use tokio_postgres::{config::Config, Error as PgError, NoTls};

pub mod connection;
use connection::PgConn;
mod manager;
use manager::PgConnManager;

/// Custom postgres connection pool.
pub type PgPool = Pool<PgConnManager<NoTls>>;

/// Used to construct a custom postgres connection pool.
pub struct PgPoolBuilder {}

impl PgPoolBuilder {
    /// Create a pool of custom connections with pre-cached prepared statements.
    pub async fn build(db_url: &str, max_size: u32) -> Result<PgPool> {
        let cfg = Config::from_str(db_url)?;
        let mgr = PgConnManager::new(cfg, NoTls);
        Pool::builder()
            .connection_customizer(Box::new(PgConnCustomizer))
            .max_size(max_size)
            .build(mgr)
            .await
            .map_err(Error::from)
    }
}

#[derive(Debug)]
struct PgConnCustomizer;

#[async_trait]
impl CustomizeConnection<PgConn, PgError> for PgConnCustomizer {
    async fn on_acquire(&self, conn: &mut PgConn) -> Result<(), PgError> {
        // Set search patch for schema support
        conn.execute(sql::SET_SEARCH_PATH, &[]).await?;
        Ok(())
    }
}

/// Map tokio postgres errors to project errors.
impl From<PgError> for Error {
    fn from(err: PgError) -> Self {
        Error::internal(err.to_string())
    }
}

/// Map bb8 errors to project errors.
impl<E: StdError> From<RunError<E>> for Error {
    fn from(err: RunError<E>) -> Self {
        match err {
            RunError::TimedOut => Error::internal("connection timed out".into()),
            RunError::User(err) => Error::internal(err.to_string()),
        }
    }
}
