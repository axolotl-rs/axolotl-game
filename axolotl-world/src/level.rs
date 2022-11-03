use axolotl_nbt::value::NameLessValue;
use axolotl_types::OwnedNameSpaceKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootWrapper {
    #[serde(rename = "Data")]
    pub data: LevelDat,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftVersion {
    pub name: String,
    pub id: i32,
    pub snapshot: bool,
    pub series: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataPacks {
    pub disabled: Vec<String>,
    pub enabled: Vec<String>,
}
impl Default for DataPacks {
    fn default() -> Self {
        Self {
            disabled: vec![],
            enabled: vec!["vanilla".to_string()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDat {
    pub version: MinecraftVersion,
    pub game_rules: HashMap<String, Value>,
    pub data_packs: DataPacks,
    #[serde(rename = "version")]
    pub version_num: i32,
    pub data_version: i32,
    pub was_modded: bool,
    pub server_brands: Vec<String>,
    pub spawn_angle: f32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub level_name: String,
    pub world_gen_settings: WorldGenSettings,
    #[serde(rename = "initialized")]
    pub initialized: bool,
    #[serde(rename = "hardcore")]
    pub hardcore: bool,
    pub game_type: i32,
    pub time: i64,
    pub last_played: i64,
    pub day_time: i64,
    pub allow_commands: bool,
    pub difficulty: i32,
    pub border_center_x: f64,
    pub border_center_z: f64,
    pub border_damage_per_block: f64,
    pub border_safe_zone: f64,
    pub border_size: f64,
    pub border_size_lerp_target: f64,
    #[serde(rename = "thundering")]
    pub thundering: bool,
    #[serde(rename = "thunderTime")]
    pub thunder_time: i32,

    #[serde(flatten)]
    pub other: HashMap<String, NameLessValue>,
}
impl Default for LevelDat {
    fn default() -> Self {
        LevelDat {
            version: Default::default(),
            game_rules: Default::default(),
            data_packs: Default::default(),
            version_num: 19133,
            data_version: 3120,
            was_modded: true,
            server_brands: vec![],
            spawn_angle: 0.0,
            spawn_x: 0,
            spawn_y: 0,
            spawn_z: 0,
            level_name: String::new(),
            world_gen_settings: Default::default(),
            initialized: false,
            hardcore: false,
            game_type: 0,
            time: 0,
            last_played: 0,
            day_time: 0,
            allow_commands: false,
            difficulty: 0,
            border_center_x: 0.0,
            border_center_z: 0.0,
            border_damage_per_block: 0.0,
            border_safe_zone: 0.0,
            border_size: 59999984.0,
            border_size_lerp_target: 59999984.0,
            thundering: false,
            thunder_time: 0,
            other: Default::default(),
        }
    }
}

pub fn default_game_rules() -> HashMap<String, Value> {
    let mut game_rules = HashMap::new();
    game_rules.insert("commandBlockOutput".to_string(), Value::Bool(false));
    game_rules.insert("doDaylightCycle".to_string(), Value::Bool(true));
    game_rules.insert("doEntityDrops".to_string(), Value::Bool(true));
    game_rules.insert("doFireTick".to_string(), Value::Bool(true));
    game_rules.insert("doMobLoot".to_string(), Value::Bool(true));
    game_rules.insert("doMobSpawning".to_string(), Value::Bool(true));
    game_rules.insert("doTileDrops".to_string(), Value::Bool(true));
    game_rules.insert("doWeatherCycle".to_string(), Value::Bool(true));
    game_rules.insert("keepInventory".to_string(), Value::Bool(false));
    game_rules.insert("logAdminCommands".to_string(), Value::Bool(true));
    game_rules.insert(
        "maxCommandChainLength".to_string(),
        Value::Number(65536.into()),
    );
    game_rules.insert("maxEntityCramming".to_string(), Value::Number(24.into()));
    game_rules.insert("mobGriefing".to_string(), Value::Bool(true));
    game_rules.insert("naturalRegeneration".to_string(), Value::Bool(true));
    game_rules.insert("randomTickSpeed".to_string(), Value::Number(3.into()));
    game_rules.insert("reducedDebugInfo".to_string(), Value::Bool(false));
    game_rules.insert("sendCommandFeedback".to_string(), Value::Bool(true));
    game_rules.insert("showDeathMessages".to_string(), Value::Bool(true));
    game_rules.insert("spawnRadius".to_string(), Value::Number(10.into()));
    game_rules.insert("spectatorsGenerateChunks".to_string(), Value::Bool(true));
    game_rules.insert("universalAnger".to_string(), Value::Bool(false));
    game_rules
}
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldGenSettings {
    pub seed: i64,
    pub dimensions: HashMap<OwnedNameSpaceKey, Dimension>,
    #[serde(default)]
    pub generate_features: bool,
    #[serde(default)]
    pub bonus_chest: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "type")]
    pub world_type: OwnedNameSpaceKey,
    pub generator: Value,
    #[serde(flatten)]
    pub other: HashMap<String, NameLessValue>,
}
