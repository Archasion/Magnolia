use async_trait::async_trait;
use builders::component::ButtonBuilder;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

use crate::components::ComponentHandler;
use crate::modals::placeholder::PlaceholderModal;
use crate::modals::ModalHandler;

pub struct PlaceholderComponent<'a> {
    #[allow(dead_code)]
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
