use anyhow::Context;
use async_trait::async_trait;
use builders::component::ActionRowBuilder;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::Interaction;
use twilight_model::guild::Permissions;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::CommandHandler;
use crate::components::verify_devforum_rank::VerifyDevForumRank;
use crate::components::ComponentHandler;

#[allow(dead_code)]
pub(crate) struct DevForumSelfRole<'a>(pub(crate) &'a Interaction);

#[async_trait]
impl CommandHandler for DevForumSelfRole<'_> {
    fn model() -> anyhow::Result<Command> {
        Ok(CommandBuilder::new(
            "devforum-self-role",
            "Send an info embed with a button to self-update DevForum roles.",
            CommandType::ChatInput,
        )
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
        .validate()
        .context("validate devforum-self-role command")?
        .build())
    }

    async fn exec(&self, state: crate::State) -> anyhow::Result<InteractionResponse> {
        let devforum_logo = ImageSource::attachment("assets/devforum-logo.png")
            .context("resolve devforum logo path")?;
        let info_embed = EmbedBuilder::new()
            .title("Developer Forum Member Role(s)")
            .description(format!(
            "Click the `Update Roles` button below to claim your developer forum member role if you meet the eligibility criteria.

- <@&{}> - Your **trust level** on the Roblox developer forum is `Member` (not to be confused with `Visitor`)
- <@&{}> - Your **trust level** on the Roblox developer forum is `Regular`
- What is the developer forum? [**Learn more**](https://help.roblox.com/hc/articles/360000240223)
- How do I \"level up\"? [**Learn more**](https://devforum.roblox.com/t/3170997)",
            state.cfg.roles.devforum_member,
            state.cfg.roles.devforum_regular
            ))
            .thumbnail(devforum_logo)
            .build();
        let action_row = ActionRowBuilder::new()
            .set_components([VerifyDevForumRank::model()?])
            .build()
            .context("build action row")?;

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .embeds([info_embed])
                    .components([action_row])
                    .build(),
            ),
        })
    }
}
