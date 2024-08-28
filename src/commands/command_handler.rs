use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::http::interaction::InteractionResponse;

#[async_trait]
pub trait CommandHandler {
    fn model() -> Command;
    async fn exec(
        command: &Box<CommandData>,
    ) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>>;
}
