use async_trait::async_trait;
use twilight_model::http::interaction::InteractionResponse;

/// Trait for implementing modals.
#[async_trait]
#[allow(dead_code)]
pub(crate) trait ModalHandler: Send {
    fn model() -> anyhow::Result<InteractionResponse>
    where
        Self: Sized;
    async fn exec(&self, state: crate::State) -> anyhow::Result<InteractionResponse>;
}

// Uncomment this when there is a modal to handle.
//
// pub(crate) async fn handle_modal(
//     interaction: &Interaction,
//     custom_id: &str,
//     state: crate::State,
// ) -> anyhow::Result<InteractionResponse> {
//     let handler: Box<dyn ModalHandler> = match custom_id {
//         unknown => anyhow::bail!("unknown modal custom id: {unknown}"),
//     };
//     handler
//         .exec(state)
//         .await
//         .with_context(|| "execute modal: {custom_id}")
// }
