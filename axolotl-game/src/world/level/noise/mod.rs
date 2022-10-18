use crate::world::chunk::{AxolotlChunk, PlacedBlock};

use crate::world::level::biome_source::BiomeSourceSettings;
use crate::{AxolotlGame, GameNoise};
use axolotl_api::game::{DataRegistries, Game, Registries, Registry};
use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::world_gen::noise::density::{DensityContext, Function};
use axolotl_api::world_gen::noise::{ChunkGenerator, NameSpaceKeyOrType, NoiseSetting};
use axolotl_api::OwnedNameSpaceKey;
use std::collections::HashMap;

pub struct ChunkContext {
    pub chunk_x: i64,
    pub chunk_z: i64,
    pub y: i16,
}
impl DensityContext for ChunkContext {
    fn get_x(&self) -> i64 {
        self.chunk_x
    }

    fn get_y(&self) -> i16 {
        self.y
    }

    fn get_z(&self) -> i64 {
        self.chunk_z
    }
}
#[derive(Debug)]
pub struct NoiseGenerator<'game> {
    game: &'game AxolotlGame,
    default_block: PlacedBlock,
    density_functions: HashMap<OwnedNameSpaceKey, Function<'static, GameNoise>>,
}

impl<'game> ChunkGenerator<'game> for NoiseGenerator<'game> {
    type PerlinNoise = GameNoise;
    type ChunkSettings = (BiomeSourceSettings, NameSpaceKeyOrType<NoiseSetting>);
    type Chunk = AxolotlChunk;
    type GameTy = AxolotlGame;

    fn new(game: &'game Self::GameTy, chunk_settings: Self::ChunkSettings) -> Self {
        let (_biome_source, settings) = chunk_settings;
        let settings = match settings {
            NameSpaceKeyOrType::NameSpaceKey(key) => game
                .data_registries()
                .get_noise_setting_registry()
                .get(&key)
                .unwrap()
                .clone(),
            NameSpaceKeyOrType::Type(ty) => ty,
        };

        let mut default_block = game
            .registries()
            .get_block_registry()
            .get(&settings.default_block.name)
            .unwrap()
            .get_default_placed_block();
        for (key, value) in settings.default_block.properties {
            default_block.state.set(key, BlockStateValue::String(value));
        }
        Self {
            game,
            default_block,
            density_functions: Default::default(),
        }
    }

    fn generate_chunk(&self, chunk_x: i64, chunk_z: i64) -> Self::Chunk {
        let chunk = AxolotlChunk {
            chunk_x,
            chunk_z,
            sections: Default::default(),
        };

        let _context = ChunkContext {
            chunk_x,
            chunk_z,
            y: 0,
        };

        return chunk;
    }
}
