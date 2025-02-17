use async_trait::async_trait;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::http::interaction::InteractionResponse;

pub(crate) mod placeholder;

/// Get all application command models.
pub(crate) fn models() -> anyhow::Result<Vec<Command>> {
    Ok(vec![placeholder::PlaceholderCommand::model()?])
}

/// Trait for implementing application commands.
#[async_trait]
pub trait CommandHandler: Send {
    fn model() -> anyhow::Result<Command>
    where
        Self: Sized;
    async fn exec(&self) -> anyhow::Result<InteractionResponse>;
}

impl<'a> TryFrom<&'a Box<CommandData>> for Box<dyn CommandHandler + 'a> {
    type Error = anyhow::Error;

    fn try_from(data: &'a Box<CommandData>) -> Result<Box<dyn CommandHandler + 'a>, Self::Error> {
        match data.name.as_str() {
            "placeholder" => Ok(Box::new(placeholder::PlaceholderCommand { data })),
            unknown => anyhow::bail!("unknown command name: {}", unknown),
        }
    }
}
