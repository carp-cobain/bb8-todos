use crate::db::{pool::connection::PgConn, sql};
use futures::future::try_join_all;
use tokio_postgres::{Error, Statement};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum StatementKey {
    SelectStories,
    SelectStory,
    InsertStory,
    DeleteStory,
}

/// Prepare and cache hot-path queries.
pub async fn cache(conn: &mut PgConn) -> Result<(), Error> {
    // Group statement keys with sql
    let queries = vec![
        (StatementKey::SelectStories, sql::stories::SELECT),
        (StatementKey::SelectStory, sql::stories::FETCH),
        (StatementKey::InsertStory, sql::stories::INSERT),
        (StatementKey::DeleteStory, sql::stories::DELETE),
    ];

    // Prepare sql statements
    let statements: Vec<_> =
        try_join_all(queries.into_iter().map(|(k, v)| prepare(conn, k, v))).await?;

    // Cache in connection state
    for (key, statement) in statements {
        conn.ps_cache.insert(key, statement);
    }

    Ok(())
}

/// Prepare a sql statement and group with a statement key
async fn prepare(
    conn: &PgConn,
    key: StatementKey,
    sql: &str,
) -> Result<(StatementKey, Statement), Error> {
    let statement = conn.prepare(sql).await?;
    let tup = (key, statement);
    Ok(tup)
}
