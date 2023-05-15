use crate::server_config::ServerConfig;
use serenity::prelude::*;
use std::sync::Arc;

mod commands;
mod database;
mod handler;
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
    let mut client = Client::builder(config.discord_token, GatewayIntents::empty())
        .event_handler(handler::Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
