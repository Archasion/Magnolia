use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::command_handler::CommandHandler;

pub struct PlaceholderCommand;

#[async_trait]
impl CommandHandler for PlaceholderCommand {
    fn data() -> Command {
        CommandBuilder::new("placeholder", "This is a placeholder command", CommandType::ChatInput).build()
    }

    async fn exec(
        _interaction: &Box<InteractionCreate>,
    ) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>> {
        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content("This is a placeholder command")
                    .build(),
            ),
        })
    }
}