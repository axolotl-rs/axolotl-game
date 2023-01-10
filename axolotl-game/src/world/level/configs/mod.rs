use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use serde::de::MapAccess;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use axolotl_api::world::WorldLocationID;
use axolotl_api::OwnedNameSpaceKey;

use crate::world::generator::ChunkSettings;

/// This is the default config for worlds. This is to make a vanilla like feel to the game.
pub static DEFAULT_VANILLA_CONFIG: &str = include_str!("vanilla.worldconfig.json");
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldsConfig {
    pub groups: Vec<WorldGrouping>,
    /// If undefined a plugin will have to handle this
    pub default: Option<WorldLocationID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalRule {
    pub to: WorldLocationID,
    pub from: WorldLocationID,
}
/// A grouping will share the same player data and resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGrouping {
    pub name: String,
    pub worlds: Vec<WorldConfig>,
    /// From world name to world uuid
    pub nether_portal_rules: Vec<PortalRule>,
    pub end_portal_rules: Vec<PortalRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub name: String,
    pub path: PathBuf,
    pub world_type: OwnedNameSpaceKey,
    pub generator: ChunkSettings,
    pub seed: Option<Value>,
    /// By default they will use the grouping resource pool however, This will force them to use a different one
    #[serde(default)]
    pub use_own_resource_pool: bool,
}
pub trait WorldGroupAccessor {
    fn world_config(&self) -> &WorldConfig;
}

#[cfg(test)]
mod tests {
    use crate::world::level::configs::DEFAULT_VANILLA_CONFIG;

    #[test]
    pub fn test_load() {
        let config: super::WorldsConfig = serde_json::from_str(DEFAULT_VANILLA_CONFIG).unwrap();
        println!("{:#?}", config);
    }
}
