use std::error::Error;

use async_trait::async_trait;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::modals::modal_handler::ModalHandler;

pub struct PlaceholderModal;

#[async_trait]
impl ModalHandler for PlaceholderModal {
	fn model() -> InteractionResponse {
		let text_field = Component::TextInput(TextInput {
			custom_id: "placeholder".to_owned(),
			label: "Placeholder".to_owned(),
			max_length: Some(256),
			min_length: None,
			placeholder: None,
			required: None,
			style: TextInputStyle::Paragraph,
			value: None,
		});

		let action_row = Component::ActionRow(ActionRow {
			components: vec![text_field],
		});

		InteractionResponse {
			kind: InteractionResponseType::Modal,
			data: Some(
				InteractionResponseDataBuilder::new()
					.title("Placeholder")
					.custom_id("placeholder")
					.components([action_row])
					.build(),
			),
		}
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
