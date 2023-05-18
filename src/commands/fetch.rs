use crate::{
    commands::{CommandError, DiscordCommand},
    context::BotContext,
    loader::scraper::Scraper,
};
use rand::Rng;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            Interaction, InteractionResponseType,
        },
        prelude::command::CommandOptionType,
    },
    prelude::*,
};

const MAX_ATTACHMENT_SIZE: usize = 7 * 1024 * 1024;

#[derive(Clone)]
pub struct FetchCommand;

#[async_trait]
impl DiscordCommand for FetchCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("fetch")
            .description("Fetch image(s) from supported source.")
            .create_option(|option| {
                option
                    .name("url")
                    .description("Source url")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }

    async fn resolve(
        &self,
        context: Context,
        command: ApplicationCommandInteraction,
        interaction: Interaction,
    ) -> Result<(), CommandError> {
        let options = &command.data.options;
        let url = match &options.get(0) {
            Some(option) => match &option.resolved {
                Some(resolved) => match resolved {
                    CommandDataOptionValue::String(url) => url,
                    _ => panic!("implementation error?"),
                },
                None => return Err(CommandError::InvalidUrl()),
            },
            None => return Err(CommandError::InvalidUrl()),
        };
        let scraper = Scraper::from_url(url.to_string());
        if scraper.is_none() {
            return Err(CommandError::UnsupportedSite());
        }
        let scraper = scraper.unwrap();
        command
            .create_interaction_response(&context.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!("Source: {}", url.to_string()))
                    })
            })
            .await?;
        let slf = self.clone();
        tokio::spawn(async move {
            tokio::task::spawn_blocking(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    slf.fetch_image(context, scraper, command, interaction)
                        .await;
                });
            });
        });
        Ok(())
    }
}

impl FetchCommand {
    async fn fetch_image(
        &self,
        context: Context,
        scraper: Scraper,
        command: ApplicationCommandInteraction,
        interaction: Interaction,
    ) {
        let mut rng = rand::thread_rng();

        let bot_context = {
            let data_read = context.data.read().await;
            data_read
                .get::<BotContext>()
                .expect("Can't found bot context")
                .clone()
        };

        let gallery = match scraper.fetch_with_context(Some(bot_context.clone())).await {
            Ok(gallery) => gallery,
            Err(err) => {
                log::error!("{:?}", err);
                return ();
            }
        };

        let works = match gallery.to_vec().await {
            Ok(works) => works,
            Err(err) => {
                log::error!("{:?}", err);
                return ();
            }
        };

        let mut batch_size: usize = 0;
        let mut batch: Vec<Vec<u8>> = Vec::new();
        for work in works {
            if work.len() > MAX_ATTACHMENT_SIZE {
                continue;
            }

            if batch_size + work.len() > MAX_ATTACHMENT_SIZE {
                let names = (0..batch.len())
                    .map(|_| format!("{}.png", rng.gen::<u64>()))
                    .collect::<Vec<_>>();
                let mut i = 0;
                let _ = &command
                    .channel_id
                    .send_files(
                        &context.http,
                        batch.iter().map(|work| {
                            i = i + 1;
                            (work.as_slice(), names[i - 1].as_ref())
                        }),
                        |m| m.content(""),
                    )
                    .await;
                batch.clear();
                batch_size = 0;
                continue;
            }
            batch.push(work);
        }

        if !batch.is_empty() {
            let names = (0..batch.len())
                .map(|_| format!("{}.png", rng.gen::<u64>()))
                .collect::<Vec<_>>();
            let mut i = 0;
            let _ = &command
                .channel_id
                .send_files(
                    &context.http,
                    batch.iter().map(|work| {
                        i = i + 1;
                        (work.as_slice(), names[i - 1].as_ref())
                    }),
                    |m| m.content(""),
                )
                .await;
        }
    }
}
