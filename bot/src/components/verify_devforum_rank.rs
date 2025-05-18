use std::sync::LazyLock;

use anyhow::Context;
use async_trait::async_trait;
use builders::component::ButtonBuilder;
use reqwest::header::{AUTHORIZATION, COOKIE};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::{Component, MessageFlags};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::marker::{GuildMarker, RoleMarker, UserMarker};
use twilight_model::id::Id;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::components::ComponentHandler;

pub(crate) struct VerifyDevForumRank<'a>(pub(crate) &'a Interaction);

static ROVER_API_KEY: LazyLock<String> =
    LazyLock::new(|| std::env::var("ROVER_API_KEY").expect("ROVER_API_KEY must be set"));
static DEVFORUM_COOKIE: LazyLock<Option<String>> =
    LazyLock::new(|| std::env::var("DEVFORUM_COOKIE").ok());

#[async_trait]
impl ComponentHandler for VerifyDevForumRank<'_> {
    fn model() -> anyhow::Result<Component> {
        ButtonBuilder::new("verify-devforum-rank", ButtonStyle::Primary)
            .label("Update Roles")
            .build()
    }

    async fn exec(&self, state: crate::State) -> anyhow::Result<InteractionResponse> {
        let guild_id = self.0.guild_id.context("get guild id")?;
        let author_id = self.0.author_id().context("get interaction author id")?;

        // Get the user's Roblox ID using their Discord ID from the RoVer verification API.
        let rover_data = match fetch_rover_data(&state.request, guild_id, author_id).await {
            Ok(data) => data,
            Err(error) => {
                tracing::warn!(?error);
                return Ok(InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("Failed to fetch your RoVer data.")
                            .flags(MessageFlags::EPHEMERAL)
                            .build(),
                    ),
                });
            },
        };

        // Get the user's Roblox username using their Roblox ID from the Roblox API.
        let roblox_data = match fetch_roblox_data(&state.request, rover_data.roblox_id).await {
            Ok(data) => data,
            Err(error) => {
                tracing::warn!(?error);
                return Ok(InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("Failed to fetch your Roblox username.")
                            .flags(MessageFlags::EPHEMERAL)
                            .build(),
                    ),
                });
            },
        };

        // Get the user's trust level using their Roblox username from the DevForum API.
        let devforum_data = match fetch_devforum_data(&state.request, &roblox_data.name).await {
            Ok(data) => data,
            Err(error) => {
                tracing::warn!(?error);
                return Ok(InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("Failed to fetch your DevForum data.")
                            .flags(MessageFlags::EPHEMERAL)
                            .build(),
                    ),
                });
            },
        };

        // Update the user's roles in the Discord server based on their trust level.
        if let Err(error) =
            update_user_roles(guild_id, author_id, &state, &devforum_data.user.trust_level).await
        {
            tracing::error!(?error);
            return Ok(InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .content("Failed to update your roles.")
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            });
        }

        // Send a success message to the user.
        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(format!(
                        "Successfully updated your roles to match your DevForum trust level: `{}`",
                        devforum_data.user.trust_level
                    ))
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
        })
    }
}

/// Fetches the Roblox ID using the Discord ID from the RoVer API.
///
/// # Arguments
///
/// * `request` - The HTTP client used to make the request.
/// * `guild_id` - The ID of the Discord server.
/// * `discord_id` - The ID of the Discord user.
///
/// # Returns
///
/// [`RoVerAPIResponse`] if the request is successful, otherwise an error.
async fn fetch_rover_data(
    request: &reqwest::Client,
    guild_id: Id<GuildMarker>,
    discord_id: Id<UserMarker>,
) -> anyhow::Result<RoVerAPIResponse> {
    let res = request
        .get(construct_rover_endpoint(guild_id, discord_id))
        .header(AUTHORIZATION, format!("Bearer {}", *ROVER_API_KEY))
        .send()
        .await
        .context("fetch rover data")?;

    if res.status().is_success() {
        res.json::<RoVerAPIResponse>()
            .await
            .context("parse rover data")
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch RoVer data for discord_id={discord_id} in guild_id={guild_id}, received status: {}",
            res.status()
        ))
    }
}

/// Fetches the Roblox username using the Roblox ID from the Roblox API.
///
/// # Arguments
///
/// * `request` - The HTTP client used to make the request.
/// * `roblox_id` - The ID of the Roblox user.
///
/// # Returns
///
/// [`RobloxAPIResponse`] if the request is successful, otherwise an error.
async fn fetch_roblox_data(
    request: &reqwest::Client,
    roblox_id: u64,
) -> anyhow::Result<RobloxAPIResponse> {
    // Construct the Roblox API endpoint using the Roblox ID and make the request.
    let res = request
        .get(construct_roblox_endpoint(roblox_id))
        .send()
        .await
        .context("fetch roblox data")?;

    // Check if the response was successful and parse the JSON data.
    if res.status().is_success() {
        res.json::<RobloxAPIResponse>()
            .await
            .context("parse roblox data")
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch Roblox data for roblox_id={roblox_id}, received status: {}",
            res.status()
        ))
    }
}

/// Fetches the DevForum trust level using the Roblox username from the DevForum API.
///
/// # Arguments
///
/// * `request` - The HTTP client used to make the request.
/// * `roblox_username` - The username of the Roblox user.
///
/// # Returns
///
/// [`DevForumAPIResponse`] if the request is successful, otherwise an error.
async fn fetch_devforum_data(
    request: &reqwest::Client,
    roblox_username: &str,
) -> anyhow::Result<DevForumAPIResponse> {
    // Construct the DevForum API endpoint using the Roblox username and make the request.
    let mut req = request.get(construct_devforum_endpoint(roblox_username));
    if let Some(cookie) = &*DEVFORUM_COOKIE {
        req = req.header(COOKIE, format!("_t={cookie}"));
    }
    let res = req.send().await.context("fetch devforum data")?;

    // Check if the response was successful and parse the JSON data.
    if res.status().is_success() {
        res.json::<DevForumAPIResponse>()
            .await
            .context("parse devforum data")
    } else {
        Err(anyhow::anyhow!(
            "Failed to fetch DevForum data for roblox_username={roblox_username}, received status: {}",
            res.status()
        ))
    }
}

/// Updates the user's roles in the Discord server based on their trust level.
///
/// # Arguments
///
/// * `guild_id` - The ID of the Discord server.
/// * `user_id` - The ID of the Discord user.
/// * `state` - The state of the bot.
/// * `trust_level` - The trust level of the user.
async fn update_user_roles(
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
    state: &crate::State,
    trust_level: &DevForumTrustLevel,
) -> anyhow::Result<()> {
    let roles = trust_level.roles(&state.cfg);
    let mut member_roles = state
        .http
        .guild_member(guild_id, user_id)
        .await
        .context("get guild member")?
        .model()
        .await?
        .roles;

    member_roles.retain(|role_id| !roles.remove.contains(role_id));

    if let Some(role_id) = roles.add {
        if !member_roles.contains(&role_id) {
            member_roles.push(role_id);
        }
    }

    state
        .http
        .update_guild_member(guild_id, user_id)
        .roles(&member_roles)
        .await
        .context("update guild member roles")?;

    Ok(())
}

/// Constructs the RoVer API endpoint URL.
fn construct_rover_endpoint(guild_id: Id<GuildMarker>, discord_id: Id<UserMarker>) -> String {
    format!("https://registry.rover.link/api/guilds/{guild_id}/discord-to-roblox/{discord_id}")
}

/// Constructs the Roblox API endpoint URL.
fn construct_roblox_endpoint(roblox_id: u64) -> String {
    format!("https://users.roblox.com/v1/users/{roblox_id}")
}

/// Constructs the DevForum API endpoint URL.
fn construct_devforum_endpoint(roblox_username: &str) -> String {
    format!("https://devforum.roblox.com/u/{roblox_username}.json")
}

#[derive(Deserialize)]
struct RoVerAPIResponse {
    #[serde(rename = "robloxId")]
    roblox_id: u64,
}

#[derive(Deserialize)]
struct RobloxAPIResponse {
    name: String,
}

#[derive(Deserialize)]
struct DevForumAPIResponse {
    user: DevForumUser,
}

#[derive(Deserialize)]
struct DevForumUser {
    trust_level: DevForumTrustLevel,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum DevForumTrustLevel {
    Visitor = 0,
    Member = 1,
    Regular = 2,
    Staff,
}

struct RoleData {
    add: Option<Id<RoleMarker>>,
    remove: Vec<Id<RoleMarker>>,
}

impl DevForumTrustLevel {
    /// Returns the roles to add and remove based on the trust level.
    fn roles(&self, cfg: &crate::Config) -> RoleData {
        match self {
            DevForumTrustLevel::Visitor => RoleData {
                add: None,
                remove: vec![cfg.roles.devforum_member, cfg.roles.devforum_regular],
            },
            DevForumTrustLevel::Member => RoleData {
                add: Some(cfg.roles.devforum_member),
                remove: vec![cfg.roles.devforum_regular],
            },
            DevForumTrustLevel::Regular | DevForumTrustLevel::Staff => RoleData {
                add: Some(cfg.roles.devforum_regular),
                remove: vec![cfg.roles.devforum_member],
            },
        }
    }
}

impl std::fmt::Display for DevForumTrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevForumTrustLevel::Visitor => write!(f, "Visitor"),
            DevForumTrustLevel::Member => write!(f, "Member"),
            DevForumTrustLevel::Regular => write!(f, "Regular"),
            DevForumTrustLevel::Staff => write!(f, "Staff"),
        }
    }
}
