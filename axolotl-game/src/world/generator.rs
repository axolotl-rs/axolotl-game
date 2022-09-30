use std::fmt;
use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, Visitor};
use serde_json::Value;

use axolotl_api::world_gen::noise::{ChunkGenerator, NoiseSetting};
use axolotl_api::world_gen::noise::density::loading::DensityLoader;

use crate::world::chunk::AxolotlChunk;
use crate::world::perlin::GameNoise;

#[derive(Debug)]
pub enum AxolotlGenerator {
    Flat(),
    Noise(),
    Debug(),
}

impl ChunkGenerator for AxolotlGenerator {
    type PerlinNoise = GameNoise;
    type ChunkSettings = ChunkSettings;
    type Chunk = AxolotlChunk;

    fn new(_chunk_settings: Self::ChunkSettings, _density: impl DensityLoader) -> Self {
        todo!()
    }

    fn generate_chunk(&self, _chunk_x: i32, _chunk_z: i32) -> Self::Chunk {
        todo!()
    }
}

#[derive(Debug)]
pub enum ChunkSettings {
    Flat {
        biome_source: Value,
        settings: NoiseSetting,
    },
    Noise(),
    Debug(),
}
struct ChunkSettingsVisitor;

impl<'de> Visitor<'de> for ChunkSettingsVisitor {
    type Value = ChunkSettings;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for ChunkSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChunkSettingsVisitor)
    }
}
