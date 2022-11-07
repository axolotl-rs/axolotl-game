use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use crate::world_gen::Precipitation;
use crate::OwnedNameSpaceKey;

pub mod parameter;
pub mod vanilla;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Carvers {
    pub air: Option<Air>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Air(Vec<OwnedNameSpaceKey>);
pub struct AirVisitor;
impl<'de> Visitor<'de> for AirVisitor {
    type Value = Air;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Tag Air with a list of carvers or one")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Air(vec![
            OwnedNameSpaceKey::from_str(v).map_err(|e| Error::custom(e.to_string()))?
        ]))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Air(vec![
            OwnedNameSpaceKey::from_str(&v).map_err(|e| Error::custom(e.to_string()))?
        ]))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut air = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(key) = seq.next_element::<String>()? {
            air.push(OwnedNameSpaceKey::from_str(&key).map_err(|e| Error::custom(e.to_string()))?);
        }
        Ok(Air(air))
    }
}
impl Serialize for Air {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0.len().cmp(&1) {
            Ordering::Less => serializer.serialize_none(),
            Ordering::Equal => serializer.serialize_str(&self.0[0].to_string()),
            Ordering::Greater => serializer.collect_seq(self.0.iter().map(|k| k.to_string())),
        }
    }
}
impl<'de> Deserialize<'de> for Air {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AirVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    pub sky_color: u32,
    pub mood_sound: MoodSound,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSound {
    pub sound: OwnedNameSpaceKey,
    pub tick_delay: u32,
    pub offset: f32,
    pub block_search_extent: u32,
}
// TODO Add Deserialization and Serialization
#[derive(Debug, Clone)]
pub struct Features {
    pub raw: GenerationStep,
    pub lakes: GenerationStep,
    pub local_modifications: GenerationStep,
    pub underground_structures: GenerationStep,
    pub surface_structures: GenerationStep,
    pub underground_ores: GenerationStep,
    pub underground_decorations: GenerationStep,
    pub fluid_springs: GenerationStep,
    pub vegetal_decorations: GenerationStep,
    pub top_layer_modifications: GenerationStep,
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
    monster: Vec<SpawnerValue>,
    creature: Vec<SpawnerValue>,
    water_creature: Vec<SpawnerValue>,
    ambient: Vec<SpawnerValue>,
    underground_water_creature: Vec<SpawnerValue>,
    water_ambient: Vec<SpawnerValue>,
    misc: Vec<SpawnerValue>,
    /// AXOLOTL!
    axolotls: Vec<SpawnerValue>,
}
