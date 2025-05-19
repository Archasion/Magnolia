use async_trait::async_trait;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::Component;

pub(crate) mod verify_devforum_rank;

/// Trait for implementing message components.
/// See the [`Component`] enum for supported components.
#[async_trait]
pub(crate) trait ComponentHandler: Send {
    fn model() -> anyhow::Result<Component>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}

pub(crate) async fn handle_component(
    cmd: &Interaction,
    custom_id: &str,
    ctx: crate::Context,
) -> anyhow::Result<()> {
    let handler: Box<dyn ComponentHandler> = match custom_id {
        "verify-devforum-rank" => Box::new(verify_devforum_rank::VerifyDevForumRank { cmd }),
        unknown => anyhow::bail!("unknown component custom id: {}", unknown),
    };
    handler.exec(ctx).await
}
