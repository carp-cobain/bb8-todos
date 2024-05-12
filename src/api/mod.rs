use axum::Router;
use std::sync::Arc;

mod ctx;
mod dto;
mod paging;
mod status;
mod story;
mod task;

pub use ctx::Ctx;

/// The http/json presentation layer
pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    /// Create a new Api.
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    /// Combine module routes into a top-level api router.
    pub fn routes(self) -> Router {
        status::routes()
            .merge(story::routes())
            .merge(task::routes())
            .with_state(self.ctx)
    }
}
