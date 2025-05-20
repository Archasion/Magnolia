use anyhow::Context;
use serde::Deserialize;
use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceValue};
use twilight_model::channel::message::Embed;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

/// Configuration for the bot.
#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    /// A mapping of role IDs to their names.
    pub(crate) roles: RoleConfig,
    /// A list of options for the FAQ command.
    faq_options: Vec<FaqOption>,
}

/// Configuration for roles.
#[derive(Deserialize, Debug)]
pub(crate) struct RoleConfig {
    pub(crate) devforum_member: Id<RoleMarker>,
    pub(crate) devforum_regular: Id<RoleMarker>,
    pub(crate) roblox_verified: Option<Id<RoleMarker>>,
}

/// Configuration for an option of the FAQ command.
#[derive(Deserialize, Debug)]
struct FaqOption {
    /// The label of the option (displayed to the user).
    label: String,
    /// The value of the option (used as the identifier).
    value: String,
    /// The embed to be sent when this option is selected.
    embed: Embed,
}

impl Config {
    /// Returns a vector of options for the FAQ command.
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

    /// Returns the embed associated with a given FAQ option value.
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

/// Loads the configuration from a YAML file.
#[tracing::instrument(ret)]
pub(crate) fn load_config() -> Result<Config, anyhow::Error> {
    let cfg_yaml = std::fs::read(config_path()).context("read config file")?;
    serde_yaml::from_slice(&cfg_yaml).context("parse config file")
}

/// Parses the config file path from command line arguments
/// or defaults to "magnolia.cfg.yml".
pub(crate) fn config_path() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "magnolia.cfg.yml".to_string())
}
