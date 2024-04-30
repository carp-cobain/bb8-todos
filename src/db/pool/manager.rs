use super::PgConn;
use async_trait::async_trait;
use bb8::ManageConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{
    config::Config,
    tls::{MakeTlsConnect, TlsConnect},
    Error, Socket,
};

/// Custom postgres connection manager.
pub struct PgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket>,
{
    inner: PostgresConnectionManager<Tls>,
}

impl<Tls> PgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket>,
{
    /// Create a new custom postgres connection manager.
    pub fn new(config: Config, tls: Tls) -> Self {
        Self {
            inner: PostgresConnectionManager::new(config, tls),
        }
    }
}

#[async_trait]
impl<Tls> ManageConnection for PgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type Connection = PgConn;
    type Error = Error;

    /// Attempts to create a new connection.
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = self.inner.connect().await?;
        Ok(PgConn::new(conn))
    }

    /// Determine whether connection is still connected.
    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.simple_query("select 1").await.map(|_| ())
    }

    /// Determine whether connection is usable.
    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.inner.has_broken(&mut conn.inner)
    }
}
