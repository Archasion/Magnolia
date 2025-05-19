use async_trait::async_trait;

/// Trait for implementing modals.
#[async_trait]
#[allow(dead_code)]
pub(crate) trait ModalHandler: Send {
    fn model() -> anyhow::Result<()>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}

// Uncomment this when there is a modal to handle.
//
// pub(crate) async fn handle_modal(
//     cmd: &Interaction,
//     custom_id: &str,
//     ctx: crate::Context,
// ) -> anyhow::Result<()> {
//     let handler: Box<dyn ModalHandler> = match custom_id {
//         unknown => anyhow::bail!("unknown modal custom id: {unknown}"),
//     };
//     handler.exec(state).await
// }
