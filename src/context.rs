use crate::server_config::ServerConfig;
use serenity::prelude::*;
use std::sync::Arc;

pub struct BotContextInterface {
    pub config: Arc<ServerConfig>,
}

pub struct BotContext;

impl TypeMapKey for BotContext {
    type Value = Arc<BotContextInterface>;
}
