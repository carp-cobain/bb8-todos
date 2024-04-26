use crate::{db::sql::statements, Error, Result};
use async_trait::async_trait;
use bb8::{CustomizeConnection, Pool, RunError};
use std::str::FromStr;
use tokio_postgres::{config::Config, Error as DbError, NoTls};

pub mod connection;
use connection::PgConn;
mod manager;
use manager::PgConnManager;

/// Custom connection pool type
pub type PgPool = Pool<PgConnManager<NoTls>>;

/// Used to construct a custom connection pool.
pub struct PoolBuilder {}

impl PoolBuilder {
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
impl CustomizeConnection<PgConn, DbError> for PgConnCustomizer {
    async fn on_acquire(&self, conn: &mut PgConn) -> Result<(), DbError> {
        // Schema support
        let sql = "set search_path to public,bb8_todos";
        conn.execute(sql, &[]).await?;
        // Prepare and cache queries in custom connection state
        statements::cache(conn).await
    }
}

/// Map tokio postgres errors to project errors.
impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        crate::Error::internal(err.to_string())
    }
}

/// Map bb8 errors to project errors.
impl<E: std::error::Error> From<RunError<E>> for crate::Error {
    fn from(err: RunError<E>) -> Self {
        match err {
            RunError::TimedOut => crate::Error::internal("connection timed out".into()),
            RunError::User(err) => crate::Error::internal(err.to_string()),
        }
    }
}
