use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, MapAccess, Visitor};

pub use min_max::MinMax;

use crate::OwnedNameSpaceKey;
use crate::world_gen::noise::density::loading::DensityLoader;
use crate::world_gen::noise::density::perlin::Perlin;

pub mod density;
mod min_max;

#[derive(Debug, Clone)]
pub enum NameSpaceKeyOrType<T> {
    NameSpaceKey(OwnedNameSpaceKey),
    Type(T),
}

struct NameSpaceKeyOrTypeVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for NameSpaceKeyOrTypeVisitor<T> {
    type Value = NameSpaceKeyOrType<T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a namespace key or type")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(NameSpaceKeyOrType::NameSpaceKey(
            OwnedNameSpaceKey::from_str(v).map_err(E::custom)?,
        ))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(NameSpaceKeyOrType::NameSpaceKey(
            OwnedNameSpaceKey::from_str(v.as_ref()).map_err(E::custom)?,
        ))
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(NameSpaceKeyOrType::Type(Deserialize::deserialize(
            serde::de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NameSpaceKeyOrType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NameSpaceKeyOrTypeVisitor(std::marker::PhantomData))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameSpaceKeyAndProperties {
    #[serde(rename = "Name")]
    pub name: OwnedNameSpaceKey,
    #[serde(rename = "Properties")]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnTarget {
    pub continentalness: MinMax,
    pub depth: f64,
    pub erosion: MinMax,
    pub humidity: MinMax,
    pub offset: f64,
    pub temperature: MinMax,
    pub weirdness: MinMax,
}

/// Fields for world generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseParameters {
    pub height: i32,
    pub min_y: i32,
    pub size_horizontal: i32,
    pub size_vertical: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseSetting {
    pub sea_level: i32,
    pub disable_mob_generation: bool,
    pub ore_veins_enabled: bool,
    pub default_block: NameSpaceKeyAndProperties,
    pub default_fluid: NameSpaceKeyAndProperties,
    pub legacy_random_source: bool,
    pub noise: NoiseParameters,
    pub spawn_target: Vec<SpawnTarget>,
}

///- Will be implemented in game impl
///
/// Example Imples will be minecraft:noise, minecraft:flat,minecraft:debug,
pub trait ChunkGenerator {
    type PerlinNoise: Perlin;
    type ChunkSettings: for<'a> Deserialize<'a>;
    type Chunk;
    fn new(chunk_settings: Self::ChunkSettings, density: impl DensityLoader) -> Self;

    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Self::Chunk;
}

/// Biome Source
/// Example impls will be minecraft:multi_noise, minecraft:the_end, minecraft:checkerboard
pub trait BiomeSource {
    /// On Implementations that have no preset value this can be a unit struct
    type Preset;

    fn new(preset: Self::Preset) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Noise {
    pub amplitudes: Vec<f64>,
    pub first_octave: i32,
}
impl From<(Vec<f64>, i32)> for Noise{
    fn from((am, octave): (Vec<f64>, i32)) -> Self {
        Self {
            amplitudes: am,
            first_octave: octave,
        }
    }
}
impl Into<(Vec<f64>, i32)> for Noise{
    fn into(self) -> (Vec<f64>, i32) {
        (self.amplitudes, self.first_octave)
    }
}