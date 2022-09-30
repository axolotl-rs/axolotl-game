use serde::{Deserialize, Serialize};

use crate::OwnedNameSpaceKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeParameter {
    pub continentalness: f64,
    pub depth: f64,
    pub erosion: f64,
    pub humidity: f64,
    pub offset: f64,
    pub temperature: f64,
    pub weirdness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeEntry {
    pub biome: OwnedNameSpaceKey,
    pub parameter: BiomeParameter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeParameters {
    pub biomes: Vec<BiomeEntry>,
}
