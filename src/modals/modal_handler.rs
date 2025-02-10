use std::error::Error;

use async_trait::async_trait;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::http::interaction::InteractionResponse;

#[async_trait]
pub trait ModalHandler {
    fn model() -> InteractionResponse;
    async fn exec(
        modal: &ModalInteractionData,
    ) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>>;
}
