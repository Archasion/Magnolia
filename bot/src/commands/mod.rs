use async_trait::async_trait;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::Interaction;

pub(crate) mod devforum_self_role;

/// Get all application command models.
pub(crate) fn models() -> anyhow::Result<Vec<Command>> {
    Ok(vec![devforum_self_role::DevForumSelfRole::model()?])
}

/// Trait for implementing application commands.
#[async_trait]
pub(crate) trait CommandHandler: Send {
    fn model() -> anyhow::Result<Command>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}

pub(crate) async fn handle_command(
    cmd: &Interaction,
    cmd_name: &str,
    ctx: crate::Context,
) -> anyhow::Result<()> {
    let handler: Box<dyn CommandHandler> = match cmd_name {
        "devforum-self-role" => Box::new(devforum_self_role::DevForumSelfRole { cmd }),
        unknown => anyhow::bail!("unknown command name: {}", unknown),
    };
    handler.exec(ctx).await
}
