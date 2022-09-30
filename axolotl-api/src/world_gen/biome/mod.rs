use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::OwnedNameSpaceKey;
use crate::world_gen::Precipitation;

mod parameter;
mod vanilla;

/// Represents a biome
pub trait Biome: Debug {
    type Precipitation: Precipitation;
    fn get_namespace(&self) -> &OwnedNameSpaceKey;
    fn carvers(&self) -> &Carvers;

    fn get_downfall(&self) -> f32;

    fn get_effects(&self) -> &Effects;

    fn get_precipitation(&self) -> &Self::Precipitation;

    fn features(&self) -> &Features;

    fn creature_spawn_probabilities(&self) -> f32;

    fn spawners(&self) -> &Spawners;
    fn temperature(&self) -> f32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Carvers {
    pub air: Vec<OwnedNameSpaceKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    fog_color: u32,
    water_color: u32,
    water_fog_color: u32,
    sky_color: u32,
    mood_sound: MoodSound,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSound {
    sound: OwnedNameSpaceKey,
    tick_delay: u32,
    offset: f32,
    block_search_extent: u32,
}
// TODO Add Deserialization and Serialization
#[derive(Debug, Clone)]
pub struct Features {
    raw: GenerationStep,
    lakes: GenerationStep,
    local_modifications: GenerationStep,
    underground_structures: GenerationStep,
    surface_structures: GenerationStep,
    underground_ores: GenerationStep,
    underground_decorations: GenerationStep,
    fluid_springs: GenerationStep,
    vegetal_decorations: GenerationStep,
    top_layer_modifications: GenerationStep,
}

pub type GenerationStep = Vec<OwnedNameSpaceKey>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnerValue {
    weight: u32,
    #[serde(rename = "minCount")]
    min_count: u32,
    #[serde(rename = "maxCount")]
    max_count: u32,
    #[serde(rename = "type")]
    namespace: OwnedNameSpaceKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spawners {
    monsters: Vec<SpawnerValue>,
    creatures: Vec<SpawnerValue>,
    water_creatures: Vec<SpawnerValue>,
    ambient: Vec<SpawnerValue>,
    underground_water_creatures: Vec<SpawnerValue>,
    water_ambient: Vec<SpawnerValue>,
    misc: Vec<SpawnerValue>,
    /// AXOLOTL!
    axolotls: Vec<OwnedNameSpaceKey>,
}
