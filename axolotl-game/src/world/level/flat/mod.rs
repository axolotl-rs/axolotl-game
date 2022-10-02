use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
    pub block: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlatSettings {
    pub biome: String,
    pub features: bool,
    pub lakes: bool,
    pub layers: Vec<Layer>,
    pub structure_overrides: Vec<String>,
}
