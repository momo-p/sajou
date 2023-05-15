use crate::commands;
use serenity::{
    async_trait,
    model::{
        application::{interaction::{Interaction, InteractionResponseType}, command::Command},
        gateway::{Activity, Ready},
    },
    prelude::*,
};
use std::sync::Arc;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}

    async fn ready(&self, context: Context, ready: Ready) {
        let registered_commands = Box::leak(Box::new(commands::get_registered()));
        for (_, registered_command) in registered_commands {
            let registered_command = Box::leak(Box::new(registered_command));
            let http = context.http.clone();
            tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(async move {
                    let command = Command::create_global_application_command(http, |command| {
                        registered_command.register(command)
                    }).await;
                })
            });
        };

        context.shard
            .set_activity(Some(Activity::listening("Taiyou no Enogubako")));
    }
}
