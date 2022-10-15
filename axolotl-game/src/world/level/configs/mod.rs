use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldsConfig {
    pub groups: Vec<WorldGrouping>,
    /// If undefined a plugin will have to handle this
    pub default: Option<Uuid>,
}

/// A grouping will share the same player data and resources
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldGrouping {
    pub worlds: HashMap<Uuid, WorldConfig>,
    /// From world name to world uuid
    pub nether_portal_rules: HashMap<Uuid, Uuid>,
    pub end_portal_rules: HashMap<Uuid, Uuid>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub name: String,
    pub path: PathBuf,
    /// By default they will use the grouping resource pool however, This will force them to use a different one
    #[serde(default)]
    pub use_own_resource_pack: bool,
}
pub trait WorldGroupAccessor {
    fn world_config(&self) -> &WorldConfig;
}
