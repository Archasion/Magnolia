use std::error::Error;

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

pub struct PlaceholderCommand;

#[async_trait]
impl CommandHandler for PlaceholderCommand {
	fn model() -> Command {
		CommandBuilder::new(
			"placeholder",
			"This is a placeholder command",
			CommandType::ChatInput,
		)
		.validate()
		.expect("failed to build command")
		.build()
	}

	async fn exec(
		_command: &CommandData,
	) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>> {
		let button_action_row = Component::ActionRow(ActionRow {
			components: vec![PlaceholderComponent::model()],
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