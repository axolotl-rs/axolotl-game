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

#[derive(Debug, Serialize, Deserialize)]
pub struct Carvers {
    pub air: Option<Air>,
}
#[derive(Debug)]
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
        if self.0.len() == 1 {
            serializer.serialize_str(&self.0[0].to_string())
        } else if self.0.len() > 1 {
            serializer.collect_seq(self.0.iter().map(|k| k.to_string()))
        } else {
            serializer.serialize_none()
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
