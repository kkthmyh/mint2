use std::path::Path;

use config::Config;
use tracing::{info, Level};
mod config;
mod service;
use service::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let path = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .unwrap_or("Not found!");
    let mut path: String = path.into();
    path.push_str("/src/config/config.conf");
    let config = Config::load(&path);
    match config {
        Ok(c) => {
            info!("***** 加载配置文件success *****");
            let inscription_without_id = InscriptionWithOutId::new(&c);
            inscription_without_id.mint().await;
        }
        Err(e) => info!("***** 加载配置文件failed: {:?}*****", e),
    }
}
