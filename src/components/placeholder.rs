use std::error::Error;

use async_trait::async_trait;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::{Button, ButtonStyle};
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::components::component_handler::ComponentHandler;

pub struct PlaceholderComponent;

#[async_trait]
impl ComponentHandler for PlaceholderComponent {
	fn model() -> Component {
		Component::Button(Button {
			custom_id: Some("placeholder".to_owned()),
			disabled: false,
			emoji: None,
			label: Some("Placeholder".to_owned()),
			style: ButtonStyle::Primary,
			url: None,
		})
	}

	async fn exec(
		_component: &Box<MessageComponentInteractionData>,
	) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>> {
		Ok(InteractionResponse {
			kind: InteractionResponseType::ChannelMessageWithSource,
			data: Some(
				InteractionResponseDataBuilder::new()
					.content("This is a placeholder component")
					.build(),
			),
		})
	}
}
