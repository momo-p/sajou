use crate::commands;
use serenity::{
    async_trait,
    model::{
        application::{
            command::Command,
            interaction::{
                application_command::ApplicationCommandInteraction, Interaction,
                InteractionResponseType,
            },
        },
        gateway::{Activity, Ready},
    },
    prelude::*,
};
use std::sync::Arc;

pub struct Handler;

async fn throw_error(
    context: &Context,
    command: &ApplicationCommandInteraction,
    err: Option<commands::CommandError>,
) {
    let mut err = err;
    if err.clone().is_some() {
        if err.clone().unwrap().is_user_fault() {
            err = match command
                .create_interaction_response(&context.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(err.unwrap().to_message())
                        })
                })
                .await
            {
                Ok(_) => None,
                Err(err) => Some(commands::CommandError::SerenityError(Arc::new(err))),
            };
        }
    }

    if err.clone().is_some() {
        if !err.clone().unwrap().is_user_fault() {
            log::error!("{:?}", err.unwrap());
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        let command = match interaction.clone() {
            Interaction::ApplicationCommand(command) => command,
            _ => return (),
        };
        let registered_commands = commands::get_registered();
        let registered_command = registered_commands.get(&command.data.name);
        if registered_command.is_none() {
            throw_error(&context, &command, Some(commands::CommandError::NotFound())).await;
            return;
        }
        let registered_command = registered_command.unwrap();
        let err: Option<commands::CommandError> = match registered_command
            .resolve(context.clone(), command.clone(), interaction.clone())
            .await
        {
            Ok(_) => None,
            Err(err) => Some(err),
        };
        throw_error(&context, &command, err).await;
    }

    async fn ready(&self, context: Context, ready: Ready) {
        let registered_commands = Box::leak(Box::new(commands::get_registered()));
        for (_, registered_command) in registered_commands {
            let registered_command = Box::leak(Box::new(registered_command));
            let http = context.http.clone();
            tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(async move {
                    let command = Command::create_global_application_command(http, |command| {
                        registered_command.register(command)
                    })
                    .await;
                })
            });
        }

        context
            .shard
            .set_activity(Some(Activity::listening("Taiyou no Enogubako")));
    }
}
