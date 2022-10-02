use std::fmt;
use std::fmt::{Debug, Formatter};

use axolotl_api::{NamespacedKey, OwnedNameSpaceKey};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::get_type;
use axolotl_api::world_gen::noise::density::loading::DensityLoader;
use axolotl_api::world_gen::noise::{ChunkGenerator, NameSpaceKeyOrType, NoiseSetting};

use crate::world::chunk::AxolotlChunk;
use crate::world::level::biome_source::BiomeSourceSettings;
use crate::world::level::flat::FlatSettings;
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
        settings: FlatSettings,
    },
    Noise {
        biome_source: BiomeSourceSettings,
        settings: NameSpaceKeyOrType<NoiseSetting>,
    },
    Debug(),
}

struct ChunkSettingsVisitor;

impl<'de> Visitor<'de> for ChunkSettingsVisitor {
    type Value = ChunkSettings;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let value = get_type!(map);
        match value.get_key() {
            "flat" => {
                let settings: FlatSettings = map.next_value()?;
                Ok(ChunkSettings::Flat { settings })
            }
            "noise" => {
                let biome_source: BiomeSourceSettings = map.next_value()?;
                let settings: NameSpaceKeyOrType<NoiseSetting> = map.next_value()?;
                Ok(ChunkSettings::Noise {
                    biome_source,
                    settings,
                })
            }
            "debug" => {
                // As of now there are no settings for the debug generator
                Ok(ChunkSettings::Debug())
            }
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Expected `type` key to be `flat`, `noise` or `debug`, got `{}`",
                    value.get_key()
                )));
            }
        }
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
