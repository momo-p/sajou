use crate::server_config::ServerConfig;
use std::sync::Arc;

mod database;
mod loader;
mod server_config;

#[tokio::main]
async fn main() {
    let config = match ServerConfig::load() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err),
    };
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database = Arc::new(
        database::bundle::Database::new()
            .await
            .unwrap_or_else(|err| panic!("{:?}", err)),
    );
    log::info!("server started!");
}
