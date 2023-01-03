use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use axolotl_noise::minecraft::random::xoroshiro::MinecraftXoroshiro128;
use log::warn;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use axolotl_api::game::{Game, Registry};
use axolotl_api::world::World;
use axolotl_api::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use axolotl_api::world_gen::noise::density::perlin::Perlin;
use axolotl_api::world_gen::noise::density::{DensityState, Function};
use axolotl_api::world_gen::noise::{ChunkGenerator, NameSpaceKeyOrType, Noise, NoiseSetting};
use axolotl_api::OwnedNameSpaceKey;

use crate::registry::SimpleRegistry;
use crate::world::chunk::AxolotlChunk;
use crate::world::level::biome_source::BiomeSourceSettings;
use crate::world::level::flat::{FlatGenerator, FlatSettings};
use crate::world::level::noise::NoiseGenerator;
use crate::world::perlin::GameNoise;
use crate::AxolotlGame;

#[derive(Debug)]
pub enum AxolotlGenerator<W: World> {
    Flat(FlatGenerator<W>),
    Noise(NoiseGenerator<W>),
    Debug(),
}

impl<W: World> ChunkGenerator for AxolotlGenerator<W> {
    type PerlinNoise = GameNoise;
    type ChunkSettings = ChunkSettings;
    type Chunk = AxolotlChunk<W>;
    type GameTy = AxolotlGame<W>;

    fn new(game: Arc<Self::GameTy>, chunk_settings: Self::ChunkSettings) -> Self {
        match chunk_settings {
            ChunkSettings::Flat { settings } => {
                AxolotlGenerator::Flat(FlatGenerator::new(game, settings))
            }
            ChunkSettings::Noise {
                settings,
                biome_source,
            } => AxolotlGenerator::Noise(NoiseGenerator::new(game, (biome_source, settings))),
            _ => unimplemented!(),
        }
    }

    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Self::Chunk {
        match self {
            AxolotlGenerator::Flat(v) => v.generate_chunk(chunk_x, chunk_z),
            AxolotlGenerator::Noise(noise) => noise.generate_chunk(chunk_x, chunk_z),
            AxolotlGenerator::Debug() => todo!(),
        }
    }

    fn generate_chunk_into(&self, chunk: &mut Self::Chunk) {
        match self {
            AxolotlGenerator::Flat(v) => v.generate_chunk_into(chunk),
            AxolotlGenerator::Noise(noise) => {
                noise.generate_chunk_into(chunk);
            }
            AxolotlGenerator::Debug() => {}
        }
    }
}

/// This setting will only be used during loading. So large values are fine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum ChunkSettings {
    #[serde(rename = "minecraft:flat")]
    Flat { settings: FlatSettings },
    #[serde(rename = "minecraft:noise")]
    Noise {
        settings: NameSpaceKeyOrType<NoiseSetting>,
        biome_source: BiomeSourceSettings,
    },
}

#[derive(Debug)]
pub struct AxolotlDensityLoader(pub(crate) SimpleRegistry<FunctionArgument>);
impl DensityLoader for AxolotlDensityLoader {
    fn register_top_level(&mut self, key: OwnedNameSpaceKey, value: FunctionArgument) {
        match &value {
            FunctionArgument::Function { .. } => {}
            FunctionArgument::Spline(_) => {}
            _ => {
                warn!("Top level function {} is not a function or spline", key);
            }
        }
        self.0.register(key.to_string(), value);
    }

    fn build_from_def<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        _game: &G,
        _def: FunctionArgument,
    ) -> Function<P> {
        todo!()
    }

    fn build_from_def_with_cache<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        _game: &G,
        _def: NameSpaceKeyOrType<FunctionArgument>,
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
        _game: &G,
        _def: FunctionArgument,
    ) -> Function<P> {
        todo!()
    }

    fn build_from_def_with_cache<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        _game: &G,
        _def: NameSpaceKeyOrType<FunctionArgument>,
    ) -> Function<P> {
        todo!()
    }
}
