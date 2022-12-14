use std::sync::Arc;

use log::warn;

use axolotl_api::game::{DataRegistries, Game, Registry};
use axolotl_api::world::World;
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::density::DensityContext;
use axolotl_api::world_gen::noise::{ChunkGenerator, NameSpaceKeyOrType, NoiseSetting};

use crate::world::chunk::AxolotlChunk;
use crate::world::level::biome_source::BiomeSourceSettings;
use crate::{AxolotlGame, GameNoise};

pub struct ChunkContext {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub y: i16,
}
impl DensityContext for ChunkContext {
    fn get_x(&self) -> i32 {
        self.chunk_x
    }

    fn get_y(&self) -> i16 {
        self.y
    }

    fn get_z(&self) -> i32 {
        self.chunk_z
    }
}
#[derive(Debug)]
pub struct Settings {
    pub noise: NoiseSetting,
    pub biome_source: BiomeSourceSettings,
}
#[derive(Debug)]
pub struct NoiseGenerator<W: World> {
    game: Arc<AxolotlGame<W>>,
    noise: NoiseSetting,
    biome_source: BiomeSourceSettings,
}

impl<W: World> ChunkGenerator for NoiseGenerator<W> {
    type PerlinNoise = GameNoise;
    type ChunkSettings = (BiomeSourceSettings, NameSpaceKeyOrType<NoiseSetting>);
    type Chunk = AxolotlChunk<W>;
    type GameTy = AxolotlGame<W>;

    fn new(game: Arc<AxolotlGame<W>>, chunk_settings: Self::ChunkSettings) -> Self {
        let (biome_source, settings) = chunk_settings;
        let settings = match settings {
            NameSpaceKeyOrType::NameSpaceKey(key) => game
                .data_registries()
                .get_noise_setting_registry()
                .get_by_namespace_key(&key)
                .unwrap()
                .clone(),
            NameSpaceKeyOrType::Type(ty) => ty,
        };

        Self {
            game,
            noise: settings,
            biome_source,
        }
    }

    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Self::Chunk {
        let mut chunk = AxolotlChunk::new(ChunkPos::new(chunk_x, chunk_z));
        self.generate_chunk_into(&mut chunk);
        chunk
    }

    fn generate_chunk_into(&self, _chunk: &mut Self::Chunk) {
        warn!("Unimplemented chunk generation");
    }
}
