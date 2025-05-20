mod commands;
mod components;
mod config;
mod modals;

use std::env;
use std::sync::Arc;

use anyhow::Context as _;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData;

use crate::config::Config;

#[derive(Clone)]
pub(crate) struct Context {
    http: Arc<HttpClient>,
    cfg: Arc<Config>,
    request: Arc<reqwest::Client>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").context("get DISCORD_TOKEN env")?;

    // Use intents to only receive guild message events.
    let shard = Shard::new(ShardId::ONE, token.clone(), Intents::empty());

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = DefaultInMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Parse the config file.
    let cfg = Arc::new(config::load_config()?);
    let req_client = Arc::new(reqwest::Client::new());

    // Initialize the state.
    let state = Context {
        http: http.clone(),
        cfg: cfg.clone(),
        request: req_client.clone(),
    };

    handle_event_wrapper(shard, cache, state).await?;
    Ok(())
}

async fn handle_event_wrapper(
    mut shard: Shard,
    cache: DefaultInMemoryCache,
    state: Context,
) -> anyhow::Result<()> {
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
        tokio::spawn(handle_event(event, state.clone()));
    }

    Ok(())
}

async fn handle_event(event: Event, ctx: Context) -> anyhow::Result<()> {
    let res: anyhow::Result<()> = match event {
        Event::Ready(client) => {
            tracing::info!(
                "the client has logged in as @{} ({})",
                client.user.name,
                client.user.id
            );

            // Publish commands every time the bot starts
            // to ensure they are always up to date.
            let global_commands = ctx
                .http
                .interaction(client.application.id)
                .set_global_commands(commands::models(ctx.clone())?.as_slice())
                .await
                .context("publish global commands")?
                .models()
                .await
                .context("get global commands")?;

            tracing::info!("published {} global commands", global_commands.len());
            Ok(())
        },
        Event::InteractionCreate(interaction) => {
            match &interaction.data {
                Some(InteractionData::ApplicationCommand(command)) => {
                    commands::handle_command(&interaction.0, command.name.as_str(), ctx.clone())
                        .await
                        .with_context(|| format!("handle command: {}", command.name))?;
                },
                Some(InteractionData::MessageComponent(component)) => {
                    components::handle_component(
                        &interaction.0,
                        component.custom_id.as_str(),
                        ctx.clone(),
                    )
                    .await
                    .with_context(|| format!("handle component: {}", component.custom_id))?;
                },
                // Uncomment this when there is a modal to handle.
                //
                // Some(InteractionData::ModalSubmit(modal)) => {
                // modals::handle_modal(&interaction.0, modal.custom_id.as_str(), state.clone())
                //     .await
                // },
                _ => anyhow::bail!("unsupported interaction type"),
            };
            Ok(())
        },
        _ => Ok(()),
    };

    if let Err(err) = res {
        tracing::error!(source = ?err, "error handling event");
    }

    Ok(())
}
