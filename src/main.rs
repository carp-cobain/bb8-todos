#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bb8_todos::{
    api::{Api, Ctx},
    config::Config,
    pool::PgPool,
};
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::runtime::Builder;

fn main() {
    // Init from env
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Load config
    let config = Arc::new(Config::default());
    tracing::debug!("Loaded config = {:?}", config);

    // Create a runtime on the main thread
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    // Wire up db connection pool
    let pool = rt.block_on(bb8_todos::pool::new(&config.db_url));
    let pool = Arc::new(pool);

    // Run a server on the main thread
    tracing::info!("Server listening on {}", config.listen_addr);
    rt.block_on(serve(config, pool));
}

async fn serve(config: Arc<Config>, pool: Arc<PgPool>) {
    let listener = config.tcp_listener();
    let ctx = Ctx::new(config, pool).await;
    let api = Api::new(Arc::new(ctx));
    axum::serve(listener, api.routes()).await.unwrap();
}
