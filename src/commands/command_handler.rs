use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::command::Command;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::InteractionResponse;

#[async_trait]
pub trait CommandHandler {
    fn data() -> Command;
    async fn exec(
        interaction: &Box<InteractionCreate>,
    ) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>>;
}
