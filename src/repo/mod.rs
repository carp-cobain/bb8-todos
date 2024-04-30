use crate::db::pool::PgPool;

mod story;
mod task;

/// A thin abstraction layer over the database schema.
/// Maps query results to domain objects.
pub struct Repo {
    pool: PgPool,
}

impl Repo {
    /// Create a new repo.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
