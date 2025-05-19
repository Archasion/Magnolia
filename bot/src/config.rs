use anyhow::Context;
use serde::Deserialize;
use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceValue};
use twilight_model::channel::message::Embed;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) roles: ConfigRoles,
    faq_options: Vec<FAQOption>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigRoles {
    pub(crate) devforum_member: Id<RoleMarker>,
    pub(crate) devforum_regular: Id<RoleMarker>,
    pub(crate) roblox_verified: Option<Id<RoleMarker>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FAQOption {
    label: String,
    value: String,
    embed: Embed,
}

impl Config {
    pub(crate) fn faq_option_choices(&self) -> Vec<CommandOptionChoice> {
        self.faq_options
            .iter()
            .map(|opt| CommandOptionChoice {
                name: opt.label.clone(),
                value: CommandOptionChoiceValue::String(opt.value.clone()),
                name_localizations: None,
            })
            .collect()
    }

    pub(crate) fn faq_option_embed<S>(&self, value: S) -> Option<Embed>
    where
        S: AsRef<str>,
    {
        self.faq_options
            .iter()
            .find(|opt| opt.value == value.as_ref())
            .map(|opt| opt.embed.clone())
    }
}

#[tracing::instrument(ret)]
pub(crate) fn load_config<S>(path: S) -> Result<Config, anyhow::Error>
where
    S: AsRef<str> + std::fmt::Debug,
{
    let cfg_yaml = std::fs::read_to_string(path.as_ref()).context("read config file")?;
    serde_yaml::from_str(&cfg_yaml).context("parse config file")
}
