use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::{Activity, Ready},
    },
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.shard
            .set_activity(Some(Activity::listening("Taiyou no Enogubako")));
    }
}
