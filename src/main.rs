mod builders;
mod commands;
mod components;
mod modals;
mod model;

use std::env;
use std::sync::Arc;

use anyhow::Context;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData;

use crate::commands::CommandHandler;
use crate::components::ComponentHandler;
use crate::modals::ModalHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").context("get DISCORD_TOKEN env")?;

    // Use intents to only receive guild message events.
    let mut shard = Shard::new(
        ShardId::ONE,
        token.clone(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = DefaultInMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Process each event as they come in.
    while let Some(item) = shard
        .next_event(
            // We only care about the `Ready` and `InteractionCreate` events.
            EventTypeFlags::from_bits_retain(
                EventTypeFlags::READY.bits() | EventTypeFlags::INTERACTION_CREATE.bits(),
            ),
        )
        .await
    {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");
            continue;
        };

        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(event: Event, http: Arc<HttpClient>) -> anyhow::Result<()> {
    match event {
        Event::Ready(client) => {
            tracing::info!(
                "the client has logged in as @{} ({})",
                client.user.name,
                client.user.id
            );

            // Publish commands every time the bot starts
            // to ensure they are always up-to-date.
            let global_commands = http
                .interaction(client.application.id)
                .set_global_commands(commands::models()?.as_slice())
                .await
                .context("publish global commands")?
                .models()
                .await
                .context("get global commands")?;

            tracing::info!("published {} global commands", global_commands.len());
        },
        Event::InteractionCreate(interaction) => {
            let response = match &interaction.data {
                Some(InteractionData::ApplicationCommand(command)) => {
                    let handler: Box<dyn CommandHandler> = command.try_into()?;
                    handler
                        .exec()
                        .await
                        .with_context(|| format!("execute command: {}", command.name))
                },
                Some(InteractionData::MessageComponent(component)) => {
                    let handler: Box<dyn ComponentHandler> = component.try_into()?;
                    handler
                        .exec()
                        .await
                        .with_context(|| format!("execute component: {}", component.custom_id))
                },
                Some(InteractionData::ModalSubmit(modal)) => {
                    let handler: Box<dyn ModalHandler> = modal.try_into()?;
                    handler
                        .exec()
                        .await
                        .with_context(|| format!("execute modal: {}", modal.custom_id))
                },
                _ => anyhow::bail!("unsupported interaction type"),
            };

            if let Ok(response) = response {
                let e = http
                    .interaction(interaction.application_id)
                    .create_response(interaction.id, &interaction.token, &response)
                    .await
                    .err();

                if let Some(e) = e {
                    tracing::error!(?e, "error creating response for interaction");
                }
            } else {
                tracing::warn!(?interaction, "no response generated for interaction");
            }
        },
        _ => {},
    }

    Ok(())
}
