use async_trait::async_trait;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

use crate::builders::component::ButtonBuilder;
use crate::components::component_handler::ComponentHandler;
use crate::modals::modal_handler::ModalHandler;
use crate::modals::placeholder::PlaceholderModal;

pub struct PlaceholderComponent<'a> {
    pub data: &'a MessageComponentInteractionData,
}

#[async_trait]
impl ComponentHandler for PlaceholderComponent<'_> {
    fn model() -> anyhow::Result<Component> {
        ButtonBuilder::new("placeholder", ButtonStyle::Primary)
            .label("Placeholder")
            .build()
    }

    async fn exec(&self) -> anyhow::Result<InteractionResponse> {
        PlaceholderModal::model()
    }
}
