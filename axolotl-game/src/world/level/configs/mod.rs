use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use serde::de::MapAccess;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
/// A WorldLocationID. This will Deserialize from either a string or a map
///
/// # Examples
/// {"group": "vanilla", "world": "overworld"}
/// "vanilla/overworld"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct WorldLocationID {
    pub group: String,
    pub world: String,
}
impl WorldLocationID {
    pub fn new(group: String, world: String) -> Self {
        Self { group, world }
    }
}
impl FromStr for WorldLocationID {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let group = split.next().ok_or("No group")?.to_string();
        let world = split.next().ok_or("No world")?.to_string();
        Ok(Self { group, world })
    }
}
impl Display for WorldLocationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.world)
    }
}
impl Serialize for WorldLocationID {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
struct WorldLocationIDVisitor;
impl<'de> serde::de::Visitor<'de> for WorldLocationIDVisitor {
    type Value = WorldLocationID;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string in the format of `group/world`")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse().map_err(serde::de::Error::custom)
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        v.parse().map_err(serde::de::Error::custom)
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map = map;
        let mut group = None;
        let mut world = None;
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "group" => group = Some(value),
                "world" => world = Some(value),
                _ => return Err(serde::de::Error::custom(format!("Unknown key `{}`", key))),
            }
        }
        Ok(WorldLocationID {
            group: group.ok_or_else(|| serde::de::Error::custom("Missing `group` key"))?,
            world: world.ok_or_else(|| serde::de::Error::custom("Missing `world` key"))?,
        })
    }
}
impl<'de> Deserialize<'de> for WorldLocationID {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(WorldLocationIDVisitor)
    }
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
