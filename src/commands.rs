use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, Interaction,
        },
        error::Error as DiscordError,
    },
    prelude::*,
};
use std::{collections::HashMap, convert, sync::Arc};

mod ping;

#[async_trait]
pub trait DiscordCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand;

    async fn resolve(
        &self,
        context: Context,
        command: ApplicationCommandInteraction,
        interaction: Interaction,
    ) -> Result<(), CommandError>;
}

pub fn get_registered<'a>() -> HashMap<String, Box<dyn DiscordCommand + Send + Sync>> {
    let mut commands: HashMap<String, Box<dyn DiscordCommand + Send + Sync>> = HashMap::new();
    commands.insert("ping".to_owned(), Box::new(ping::PingCommand));
    commands
}

#[derive(Debug, Clone)]
pub enum CommandError {
    DiscordError(DiscordError),
    SerenityError(Arc<SerenityError>),
    NotFound(),
}

impl CommandError {
    pub fn is_user_fault(&self) -> bool {
        match self {
            CommandError::DiscordError(_) => false,
            CommandError::SerenityError(_) => false,
            _ => true,
        }
    }

    pub fn to_message(&self) -> String {
        match self {
            CommandError::NotFound() => "Command not found",
            _ => "",
        }
        .to_owned()
    }
}

impl convert::From<DiscordError> for CommandError {
    fn from(err: DiscordError) -> Self {
        CommandError::DiscordError(err)
    }
}

impl convert::From<SerenityError> for CommandError {
    fn from(err: SerenityError) -> Self {
        CommandError::SerenityError(Arc::new(err))
    }
}
