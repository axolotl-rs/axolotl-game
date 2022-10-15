use axolotl_nbt::value::{NameLessValue, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    pub name: String,
    pub id: i32,
    pub snapshot: bool,
    pub series: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DataPacks {
    pub disabled: Vec<String>,
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDat {
    pub version: Version,
    pub game_rules: Vec<Value>,
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
    #[serde(rename = "Data")]
    pub other: HashMap<String, NameLessValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldGenSettings<V> {
    pub seed: i64,
    pub dimensions: Vec<Dimension<V>>,
    #[serde(default)]
    pub generate_features: bool,
    #[serde(default)]
    pub bonus_chest: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension<V> {
    pub generator: Value,
    #[serde(flatten)]
    pub other: V,
}
