use crate::commands::{CommandError, DiscordCommand};
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, Interaction, InteractionResponseType,
    },
    prelude::*,
};

pub struct PingCommand;

#[async_trait]
impl DiscordCommand for PingCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }

    async fn resolve(
        &self,
        context: Context,
        command: ApplicationCommandInteraction,
        interaction: Interaction,
    ) -> Result<(), CommandError> {
        command
            .create_interaction_response(&context.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("pong"))
            })
            .await?;
        Ok(())
    }
}
