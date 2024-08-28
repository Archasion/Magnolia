use std::error::Error;

use async_trait::async_trait;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

use crate::components::builders::ButtonBuilder;
use crate::components::component_handler::ComponentHandler;
use crate::modals::modal_handler::ModalHandler;
use crate::modals::placeholder::PlaceholderModal;

pub struct PlaceholderComponent;

#[async_trait]
impl ComponentHandler for PlaceholderComponent {
	fn model() -> Component {
		ButtonBuilder::new(ButtonStyle::Primary)
			.label("Placeholder")
			.custom_id("placeholder")
			.build()
	}

	async fn exec(
		_component: &Box<MessageComponentInteractionData>,
	) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>> {
		Ok(PlaceholderModal::model())
	}
}
