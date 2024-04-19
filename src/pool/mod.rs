use std::collections::BTreeMap;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use bb8::{CustomizeConnection, ManageConnection, Pool, RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{
    config::Config,
    tls::{MakeTlsConnect, TlsConnect},
    Client, Error, NoTls, Socket, Statement,
};

pub mod statements;
use statements::{
    StatementKey::{self, DeleteStory, InsertStory, SelectStories, SelectStory},
    Statements,
};

/// Custom connection pool type
pub type PgPool = Pool<CustomPgConnManager<NoTls>>;

/// Create a pool with custom connections that cache prepared statements.
pub async fn new(db_url: &str) -> PgPool {
    let config = Config::from_str(db_url).unwrap();
    let pg_mgr = CustomPgConnManager::new(config, NoTls);
    Pool::builder()
        .connection_customizer(Box::new(Customizer))
        .max_size(num_cpus::get() as u32)
        .build(pg_mgr)
        .await
        .expect("build pool error")
}

#[derive(Debug)]
struct Customizer;

#[async_trait]
impl CustomizeConnection<CustomPgConn, Error> for Customizer {
    async fn on_acquire(&self, conn: &mut CustomPgConn) -> Result<(), Error> {
        // Prepare and cache queries (validates sql as well)
        let stmts = Statements::prepare(conn).await;
        conn.ps_cache.insert(SelectStories, stmts.select_stories);
        conn.ps_cache.insert(SelectStory, stmts.select_story);
        conn.ps_cache.insert(InsertStory, stmts.insert_story);
        conn.ps_cache.insert(DeleteStory, stmts.delete_story);
        Ok(())
    }
}

pub struct CustomPgConn {
    inner: Client,
    pub ps_cache: BTreeMap<StatementKey, Statement>,
}

impl CustomPgConn {
    fn new(inner: Client) -> Self {
        Self {
            inner,
            ps_cache: Default::default(),
        }
    }
}

impl Deref for CustomPgConn {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct CustomPgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket>,
{
    inner: PostgresConnectionManager<Tls>,
}

impl<Tls> CustomPgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket>,
{
    pub fn new(config: Config, tls: Tls) -> Self {
        Self {
            inner: PostgresConnectionManager::new(config, tls),
        }
    }
}

#[async_trait]
impl<Tls> ManageConnection for CustomPgConnManager<Tls>
where
    Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    type Connection = CustomPgConn;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = self.inner.connect().await?;
        Ok(CustomPgConn::new(conn))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.simple_query("").await.map(|_| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.inner.has_broken(&mut conn.inner)
    }
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::internal(err.to_string())
    }
}

impl<E: std::error::Error> From<RunError<E>> for crate::Error {
    fn from(err: RunError<E>) -> Self {
        match err {
            RunError::TimedOut => crate::Error::internal("connection timed out".into()),
            RunError::User(err) => crate::Error::internal(err.to_string()),
        }
    }
}
