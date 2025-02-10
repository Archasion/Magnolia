use async_trait::async_trait;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::http::interaction::InteractionResponse;

use crate::modals;

#[async_trait]
pub trait ModalHandler: Send {
    fn model() -> anyhow::Result<InteractionResponse>
    where
        Self: Sized;
    async fn exec(&self) -> anyhow::Result<InteractionResponse>;
}

impl<'a> TryFrom<&'a ModalInteractionData> for Box<dyn ModalHandler + 'a> {
    type Error = anyhow::Error;

    fn try_from(data: &'a ModalInteractionData) -> Result<Box<dyn ModalHandler + 'a>, Self::Error> {
        match data.custom_id.as_str() {
            "placeholder" => Ok(Box::new(modals::placeholder::PlaceholderModal { data })),
            unknown => anyhow::bail!("unknown modal custom id: {}", unknown),
        }
    }
}
