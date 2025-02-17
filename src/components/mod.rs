use async_trait::async_trait;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

pub(crate) mod placeholder;

/// Trait for implementing message components.
/// See the [`Component`] enum for supported components.
#[async_trait]
pub trait ComponentHandler: Send {
    fn model() -> anyhow::Result<Component>
    where
        Self: Sized;
    async fn exec(&self) -> anyhow::Result<InteractionResponse>;
}

impl<'a> TryFrom<&'a Box<MessageComponentInteractionData>> for Box<dyn ComponentHandler + 'a> {
    type Error = anyhow::Error;

    fn try_from(
        data: &'a Box<MessageComponentInteractionData>,
    ) -> Result<Box<dyn ComponentHandler + 'a>, Self::Error> {
        match data.custom_id.as_str() {
            "placeholder" => Ok(Box::new(placeholder::PlaceholderComponent {
                data,
            })),
            unknown => anyhow::bail!("unknown component custom id: {}", unknown),
        }
    }
}
