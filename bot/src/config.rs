use anyhow::Context;
use serde::Deserialize;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) roles: ConfigRoles,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigRoles {
    pub(crate) devforum_member: Id<RoleMarker>,
    pub(crate) devforum_regular: Id<RoleMarker>,
    pub(crate) roblox_verified: Option<Id<RoleMarker>>,
}

#[tracing::instrument(ret)]
pub(crate) fn load_config<S>(path: S) -> Result<Config, anyhow::Error>
where
    S: AsRef<str> + std::fmt::Debug,
{
    let cfg_yaml = std::fs::read_to_string(path.as_ref()).context("read config file")?;
    serde_yaml::from_str(&cfg_yaml).context("parse config file")
}
