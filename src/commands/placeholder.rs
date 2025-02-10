use anyhow::Context;
use async_trait::async_trait;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::channel::message::component::ActionRow;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::command_handler::CommandHandler;
use crate::components::component_handler::ComponentHandler;
use crate::components::placeholder::PlaceholderComponent;

pub struct PlaceholderCommand<'a> {
    pub data: &'a CommandData,
}

#[async_trait]
impl CommandHandler for PlaceholderCommand<'_> {
    fn model() -> anyhow::Result<Command> {
        Ok(CommandBuilder::new(
            "placeholder",
            "This is a placeholder command",
            CommandType::ChatInput,
        )
        .validate()
        .context("validate application command")?
        .build())
    }

    async fn exec(&self) -> anyhow::Result<InteractionResponse> {
        let button_action_row = Component::ActionRow(ActionRow {
            components: vec![PlaceholderComponent::model()?],
        });

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content("This is a placeholder command")
                    .components([button_action_row])
                    .build(),
            ),
        })
    }
}
