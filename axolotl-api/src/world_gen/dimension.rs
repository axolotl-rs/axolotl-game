use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub max_inclusive: i64,
    pub min_inclusive: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterSpawnLightLevel {
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
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
