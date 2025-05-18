use anyhow::Context;
use async_trait::async_trait;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

pub(crate) mod verify_devforum_rank;

/// Trait for implementing message components.
/// See the [`Component`] enum for supported components.
#[async_trait]
pub(crate) trait ComponentHandler: Send {
    fn model() -> anyhow::Result<Component>
    where
        Self: Sized;
    async fn exec(&self, state: crate::State) -> anyhow::Result<InteractionResponse>;
}

pub(crate) async fn handle_component(
    interaction: &Interaction,
    custom_id: &str,
    state: crate::State,
) -> Result<InteractionResponse, anyhow::Error> {
    let handler: Box<dyn ComponentHandler> = match custom_id {
        "verify-devforum-rank" => Box::new(verify_devforum_rank::VerifyDevForumRank(interaction)),
        unknown => anyhow::bail!("unknown component custom id: {}", unknown),
    };
    handler
        .exec(state)
        .await
        .with_context(|| format!("execute component: {custom_id}"))
}
