use std::sync::Arc;

use rust_sut_courses_api::{config::config_loader, infrastructure::axum_http::http_serve::start};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV {}", e);
            std::process::exit(1)
        }
    };

    info!("ENV has been loaded🎉");

    start(Arc::new(dotenvy_env))
        .await
        .expect("Failed to start server ❌")
}
