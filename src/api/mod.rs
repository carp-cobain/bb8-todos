use crate::{config::Config, db::pool::PoolBuilder, repo::Repo, Result};
use axum::Router;
use std::sync::Arc;

mod status;
mod story;

pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    pub fn routes(self) -> Router {
        status::routes().merge(story::routes()).with_state(self.ctx)
    }
}

#[derive(Clone)]
pub struct Ctx {
    // TODO: Drivers and use-cases go here
    pub repo: Arc<Repo>,
}

impl Ctx {
    pub async fn init_from_config(config: Arc<Config>) -> Result<Self> {
        let pool = PoolBuilder::build(&config.db_url, config.db_max_pool_size).await?;
        let repo = Arc::new(Repo::new(pool));
        Ok(Self { repo })
    }
}
