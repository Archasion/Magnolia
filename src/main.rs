mod commands;
mod components;
mod modals;

use std::env;
use std::error::Error;
use std::sync::Arc;

use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData;
use twilight_model::http::interaction::InteractionResponse;

use crate::commands::command_handler::CommandHandler;
use crate::components::component_handler::ComponentHandler;
use crate::modals::modal_handler::ModalHandler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Initialize the tracing subscriber.
	tracing_subscriber::fmt::init();

	let token = env::var("DISCORD_TOKEN")?;

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

async fn handle_event<'a>(
	event: Event,
	http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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
				.set_global_commands(&[commands::placeholder::PlaceholderCommand::model()])
				.await?
				.models()
				.await?;

			tracing::info!("published {} global commands", global_commands.len());
		},
		Event::InteractionCreate(interaction) => {
			let response: Option<InteractionResponse> = match &interaction.data {
				Some(InteractionData::ApplicationCommand(command)) => match command.name.as_str() {
					"placeholder" => {
						Some(commands::placeholder::PlaceholderCommand::exec(&command).await?)
					},
					_ => None,
				},
				Some(InteractionData::MessageComponent(component)) => {
					match component.custom_id.as_str() {
						"placeholder" => Some(
							components::placeholder::PlaceholderComponent::exec(&component).await?,
						),
						_ => None,
					}
				},
				Some(InteractionData::ModalSubmit(modal)) => match modal.custom_id.as_str() {
					"placeholder" => {
						Some(modals::placeholder::PlaceholderModal::exec(&modal).await?)
					},
					_ => None,
				},
				_ => None,
			};

			if let Some(response) = response {
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
