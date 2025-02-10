use twilight_model::application::command::Command;

use crate::commands::command_handler::CommandHandler;

pub(crate) mod command_handler;
pub(crate) mod placeholder;

/// Get all command models.
pub(crate) fn models() -> anyhow::Result<Vec<Command>> {
    Ok(vec![placeholder::PlaceholderCommand::model()?])
}
