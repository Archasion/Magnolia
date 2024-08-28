use std::error::Error;

use async_trait::async_trait;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::TextInputStyle;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::modals::builders::{ModalBuilder, TextInputBuilder};
use crate::modals::modal_handler::ModalHandler;

pub struct PlaceholderModal;

#[async_trait]
impl ModalHandler for PlaceholderModal {
	fn model() -> InteractionResponse {
		let text_field =
			TextInputBuilder::new("Placeholder", "placeholder", TextInputStyle::Paragraph)
				.max_length(256)
				.build();

		ModalBuilder::new()
			.title("Placeholder")
			.custom_id("placeholder")
			.add_component(text_field)
			.build()
	}

	async fn exec(
		modal: &ModalInteractionData,
	) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>> {
		// The first component in the first action row is always present
		// and is required, so we can call unwrap() on it
		let input = modal.components[0].components[0].value.as_ref().unwrap();

		Ok(InteractionResponse {
			kind: InteractionResponseType::ChannelMessageWithSource,
			data: Some(
				InteractionResponseDataBuilder::new()
					.content(format!("> {}", input))
					.build(),
			),
		})
	}
}
