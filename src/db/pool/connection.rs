use crate::Result;
use std::ops::Deref;
use std::{collections::BTreeMap, ops::DerefMut};
use tokio_postgres::{Client, Statement};

/// Custom postgres connection with prepared statement cache.
/// Prepared statments must be executed by the client that created them.
pub struct PgConn {
    pub inner: Client,
    pub ps_cache: BTreeMap<String, Statement>,
}

impl PgConn {
    /// Create a new custom postgres connection.
    pub fn new(inner: Client) -> Self {
        Self {
            inner,
            ps_cache: BTreeMap::new(),
        }
    }

    /// Helper to prepare and cache sql statements.
    pub async fn prepare_cache(&mut self, sql: &str) -> Result<Statement> {
        match self.ps_cache.get(sql) {
            Some(ps) => Ok(ps.to_owned()),
            None => {
                let stmt = self.prepare(sql).await?;
                self.ps_cache.insert(sql.to_string(), stmt.clone());
                Ok(stmt)
            }
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

// Need this for transactions
impl DerefMut for PgConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
