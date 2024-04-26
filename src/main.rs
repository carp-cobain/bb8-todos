#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bb8_todos::{
    api::{Api, Ctx},
    config::Config,
};
use dotenvy::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Init from env
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Load config
    let config = Arc::new(Config::default());
    tracing::debug!("Loaded config = {:?}", config);

    // Set up api
    let ctx = Ctx::init_from_config(Arc::clone(&config)).await.unwrap();
    let api = Api::new(Arc::new(ctx));

    // Run a server on the main thread
    tracing::info!("Server listening on {}", config.listen_addr);
    axum::serve(config.tcp_listener().await, api.routes())
        .await
        .unwrap();
}
