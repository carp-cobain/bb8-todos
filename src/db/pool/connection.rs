use crate::{db::sql::statements::StatementKey, Error, Result};
use std::collections::BTreeMap;
use std::ops::Deref;
use tokio_postgres::{Client, Statement};

/// Custom postgres connection with prepared statement cache.
/// Prepared statments must be executed by the client that created them.
pub struct PgConn {
    pub inner: Client,
    pub ps_cache: BTreeMap<StatementKey, Statement>,
}

impl PgConn {
    /// Create a new custom postgres connection.
    pub fn new(inner: Client) -> Self {
        Self {
            inner,
            ps_cache: BTreeMap::new(),
        }
    }

    /// Helper to get cached prepared statements.
    pub fn get_statement(&self, key: &StatementKey) -> Result<&Statement> {
        match self.ps_cache.get(key) {
            Some(ps) => Ok(ps),
            None => Err(Error::internal(format!(
                "prepared statement not found, {:?}",
                key
            ))),
        }
    }
}

/// Deref pointer calls to the inner tokio postgres client.
impl Deref for PgConn {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
