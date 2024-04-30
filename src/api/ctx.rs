use crate::{
    config::Config,
    db::pool::{PgPool, PgPoolBuilder},
    repo::Repo,
    Result,
};
use std::sync::Arc;

/// Repo, drivers, and use-cases for use in API routes.
#[derive(Clone)]
pub struct Ctx {
    pub repo: Arc<Repo>,
}

impl Ctx {
    /// Initialize repo, drivers, and use-cases from config.
    pub async fn init_from_config(config: Arc<Config>) -> Result<Self> {
        let pool: PgPool = PgPoolBuilder::build(&config.db_url, config.db_max_pool_size).await?;
        let repo = Arc::new(Repo::new(pool));
        Ok(Self { repo })
    }
}
