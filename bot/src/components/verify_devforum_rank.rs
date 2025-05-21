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

pub(crate) struct VerifyDevForumRank<'a> {
    pub(crate) cmd: &'a Interaction,
}

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

    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()> {
        let guild_id = self.cmd.guild_id.context("get guild id")?;
        let author_id = self.cmd.author_id().context("get interaction author id")?;
        let member_roles = ctx
            .http
            .guild_member(guild_id, author_id)
            .await
            .context("get guild member")?
            .model()
            .await?
            .roles;

        // Respond early if the user doesn't have the verified role
        // this is a quick check to avoid unnecessary API calls
        if let Some(r_id) = ctx.cfg.roles.roblox_verified {
            if !member_roles.contains(&r_id) {
                ctx.http
                    .interaction(self.cmd.application_id)
                    .create_response(self.cmd.id, &self.cmd.token, &InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(
                            InteractionResponseDataBuilder::new()
                                .flags(MessageFlags::EPHEMERAL)
                                .content("You must be verified to use this interaction.")
                                .build(),
                        ),
                    })
                    .await
                    .context("respond to interaction")?;
                return Ok(());
            }
        }

        // Defer the interaction response since the API calls may take some time
        ctx.http
            .interaction(self.cmd.application_id)
            .create_response(self.cmd.id, &self.cmd.token, &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            })
            .await
            .context("defer interaction response")?;

        // Respond to the interaction
        let response = get_response_content(&ctx, guild_id, author_id, member_roles).await;
        ctx.http
            .interaction(self.cmd.application_id)
            .update_response(&self.cmd.token)
            .content(Some(&response))
            .await
            .context("edit interaction response")?;

        Ok(())
    }
}

async fn get_response_content(
    ctx: &crate::Context,
    guild_id: Id<GuildMarker>,
    author_id: Id<UserMarker>,
    member_roles: Vec<Id<RoleMarker>>,
) -> String {
    // Get the user's Roblox ID using their Discord ID from the RoVer verification API.
    let rover_data = match fetch_rover_data(&ctx.request, guild_id, author_id).await {
        Ok(data) => data,
        Err(error) => {
            tracing::warn!(?error);
            return "Failed to fetch your RoVer data.".to_string();
        },
    };

    // Get the user's Roblox username using their Roblox ID from the Roblox API.
    let roblox_data = match fetch_roblox_data(&ctx.request, rover_data.roblox_id).await {
        Ok(data) => data,
        Err(error) => {
            tracing::warn!(?error);
            return "Failed to fetch your Roblox username.".to_string();
        },
    };

    // Get the user's trust level using their Roblox username from the DevForum API.
    let devforum_data = match fetch_devforum_data(&ctx.request, &roblox_data.name).await {
        Ok(data) => data,
        Err(error) => {
            tracing::warn!(?error);
            return "Failed to fetch your DevForum data.".to_string();
        },
    };

    // Update the user's roles in the Discord server based on their trust level.
    match update_user_roles(
        guild_id,
        author_id,
        ctx,
        &devforum_data.user.trust_level,
        member_roles,
    )
    .await
    {
        Ok(()) => format!(
            "Successfully updated your roles to match your DevForum trust level: `{}`",
            devforum_data.user.trust_level
        ),
        Err(error) => {
            tracing::error!(?error);
            "Failed to update your roles.".to_string()
        },
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
    let endpoint = construct_devforum_endpoint(roblox_username);

    // Attempt request without the cookie first
    let res = request.get(&endpoint).send().await?;
    if res.status().is_success() {
        if let Ok(data) = res.json::<DevForumAPIResponse>().await {
            return Ok(data);
        }
    } else {
        anyhow::bail!(
            "Failed to fetch DevForum data for roblox_username={roblox_username} without cookie, received status: {}",
            res.status()
        )
    }

    // Return early if the cookie is not set
    if DEVFORUM_COOKIE.is_none() {
        anyhow::bail!(
            "Failed to fetch DevForum data for roblox_username={roblox_username} without cookie"
        )
    }

    // If the request fails, try again with the cookie
    let res = request
        .get(endpoint)
        .header(COOKIE, format!("_t={}", *DEVFORUM_COOKIE.as_ref().unwrap()))
        .send()
        .await?;

    if res.status().is_success() {
        res.json::<DevForumAPIResponse>()
            .await
            .context("parse devforum data with cookie")
    } else {
        anyhow::bail!(
            "Failed to fetch DevForum data for roblox_username={roblox_username} with cookie, received status: {}",
            res.status()
        )
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
    state: &crate::Context,
    trust_level: &DevForumTrustLevel,
    mut member_roles: Vec<Id<RoleMarker>>,
) -> anyhow::Result<()> {
    let roles = trust_level.roles(&state.cfg);
    // Remove the roles that are no longer applicable
    member_roles.retain(|role_id| !roles.remove.contains(role_id));

    // Add the role if it is not already present
    if let Some(role_id) = roles.add {
        if !member_roles.contains(&role_id) {
            member_roles.push(role_id);
        }
    }

    // Update the guild member with the new roles
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
