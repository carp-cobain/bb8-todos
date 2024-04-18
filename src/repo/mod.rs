use crate::pool::PgPool;
use std::sync::Arc;

mod story;

pub struct Repo {
    pool: Arc<PgPool>,
}

impl Repo {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
