pub mod light_limit;

use crate::world_gen::dimension::light_limit::UniformValue;
use axolotl_types::OwnedNameSpaceKey;
use serde::de::{DeserializeSeed, Error};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum MonsterSpawnLightLevel {
    Constant(i32),
    Uniform(UniformValue),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dimension {
    pub ambient_light: f64,
    pub bed_works: bool,
    pub coordinate_scale: f64,
    pub effects: String,
    pub has_ceiling: bool,
    pub has_raids: bool,
    pub has_skylight: bool,
    pub height: i64,
    pub infiniburn: String,
    pub logical_height: i64,
    pub min_y: i64,
    pub monster_spawn_block_light_limit: i64,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub natural: bool,
    pub piglin_safe: bool,
    pub respawn_anchor_works: bool,
    #[serde(rename = "ultrawarm")]
    pub ultra_warm: bool,
}
#[cfg(test)]
pub mod tests {
    use crate::world_gen::dimension::light_limit::UniformValue;
    use crate::world_gen::dimension::MonsterSpawnLightLevel;

    #[test]
    pub fn test() {
        let light = MonsterSpawnLightLevel::Uniform(UniformValue {
            min_inclusive: 0,
            max_inclusive: 0,
        });
        let json = serde_json::to_string(&light).unwrap();
        println!("{}", json);
    }
}
