use async_trait::async_trait;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::TextInputStyle;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::builders::component::ActionRowBuilder;
use crate::builders::modal::{ModalBuilder, TextInputBuilder};
use crate::modals::modal_handler::ModalHandler;

pub struct PlaceholderModal<'a> {
    pub data: &'a ModalInteractionData,
}

#[async_trait]
impl ModalHandler for PlaceholderModal<'_> {
    fn model() -> anyhow::Result<InteractionResponse> {
        let text_input =
            TextInputBuilder::new("Placeholder", "placeholder", TextInputStyle::Paragraph)
                .max_length(256)
                .build()?;

        let action_row = ActionRowBuilder::new().add_component(text_input).build()?;

        ModalBuilder::new("Placeholder", "placeholder")
            .add_component(action_row)
            .build()
    }

    async fn exec(&self) -> anyhow::Result<InteractionResponse> {
        // The first component in the first action row is always present
        // and is required, so we can call unwrap() on it
        let input = self.data.components[0].components[0]
            .value
            .as_ref()
            .unwrap();

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
