use anyhow::Context;
use async_trait::async_trait;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::Interaction;
use twilight_model::http::interaction::InteractionResponse;

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
    async fn exec(&self, state: crate::State) -> anyhow::Result<InteractionResponse>;
}

pub(crate) async fn handle_command(
    interaction: &Interaction,
    command_name: &str,
    state: crate::State,
) -> anyhow::Result<InteractionResponse> {
    let handler: Box<dyn CommandHandler> = match command_name {
        "devforum-self-role" => Box::new(devforum_self_role::DevForumSelfRole(interaction)),
        unknown => anyhow::bail!("unknown command name: {}", unknown),
    };
    handler
        .exec(state)
        .await
        .with_context(|| format!("execute command: {command_name}"))
}
