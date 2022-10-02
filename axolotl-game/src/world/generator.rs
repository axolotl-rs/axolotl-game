use axolotl_noise::minecraft::random::xoroshiro::MinecraftXoroshiro128;
use log::warn;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

use axolotl_api::game::Game;
use axolotl_api::{NamespacedKey, OwnedNameSpaceKey};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{get_type, AxolotlGame};
use axolotl_api::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use axolotl_api::world_gen::noise::density::perlin::Perlin;
use axolotl_api::world_gen::noise::density::{DensityState, Function};
use axolotl_api::world_gen::noise::{ChunkGenerator, NameSpaceKeyOrType, Noise, NoiseSetting};

use crate::world::chunk::AxolotlChunk;
use crate::world::level::biome_source::BiomeSourceSettings;
use crate::world::level::flat::FlatSettings;
use crate::world::level::noise::NoiseGenerator;
use crate::world::perlin::GameNoise;

#[derive(Debug)]
pub enum AxolotlGenerator<'game> {
    Flat(),
    Noise(NoiseGenerator<'game>),
    Debug(),
}

impl<'game> ChunkGenerator<'_> for AxolotlGenerator<'game> {
    type PerlinNoise = GameNoise;
    type ChunkSettings = ChunkSettings;
    type Chunk = AxolotlChunk;
    type GameTy = AxolotlGame;

    fn new(game: &Self::GameTy, chunk_settings: Self::ChunkSettings) -> Self {
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
#[derive(Debug)]
pub struct AxolotlDensityLoader {
    pub unloaded: HashMap<OwnedNameSpaceKey, FunctionArgument>,
}
impl DensityLoader for AxolotlDensityLoader {
    fn register_top_level(&mut self, key: OwnedNameSpaceKey, value: FunctionArgument) {
        match &value {
            FunctionArgument::Function { .. } => {}
            FunctionArgument::Spline(_) => {}
            _ => {
                warn!("Top level function {} is not a function or spline", key);
            }
        }
        self.unloaded.insert(key, value);
    }

    fn build_from_def<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: FunctionArgument,
    ) -> Function<P> {
        todo!()
    }

    fn build_from_def_with_cache<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: NameSpaceKeyOrType<FunctionArgument>,
    ) -> Function<P> {
        todo!()
    }
}
#[derive(Debug)]
pub struct AxolotlDensityState<'function, 'game> {
    pub seed: [u8; 16],
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub functions: &'function HashMap<OwnedNameSpaceKey, Function<'function, GameNoise>>,
    pub density_loader: &'game AxolotlDensityLoader,
}
impl<'function, 'game> DensityState for AxolotlDensityState<'function, 'game> {
    type Random = MinecraftXoroshiro128;
    type Perlin = GameNoise;

    fn seed(&self) -> [u8; 16] {
        self.seed
    }

    fn get_random(&self) -> Self::Random {
        todo!()
    }

    fn get_perlin(&self) -> &Self::Perlin {
        todo!()
    }

    fn build_from_def<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: FunctionArgument,
    ) -> Function<P> {
        todo!()
    }

    fn build_from_def_with_cache<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: NameSpaceKeyOrType<FunctionArgument>,
    ) -> Function<P> {
        todo!()
    }
}
