use crate::world::chunk::placed_block::PlacedBlock;
use crate::world::chunk::AxolotlChunk;
use crate::world::perlin::GameNoise;
use crate::AxolotlGame;
use axolotl_api::game::{Game, Registry};
use axolotl_api::world::BlockPosition;
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_api::NamespacedId;
use axolotl_items::blocks::MinecraftBlock;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
    pub block: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlatSettings {
    pub biome: String,
    pub features: bool,
    pub lakes: bool,
    pub layers: Vec<Layer>,
    pub structure_overrides: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct LoadedLayer {
    pub block: MinecraftBlock<AxolotlGame>,
    pub height: i16,
}

#[derive(Debug, Clone)]
pub struct FlatGenerator {
    pub settings: FlatSettings,
    pub layers: Vec<LoadedLayer>,
    pub game: Arc<AxolotlGame>,
}

impl ChunkGenerator<'_> for FlatGenerator {
    type PerlinNoise = GameNoise;
    type ChunkSettings = FlatSettings;
    type Chunk = AxolotlChunk;
    type GameTy = AxolotlGame;

    fn new(game: Arc<AxolotlGame>, settings: FlatSettings) -> Self {
        let mut layers = Vec::new();
        for layer in settings.layers.iter() {
            let block = game
                .registries
                .blocks
                .get_by_namespace(&layer.block)
                .unwrap_or_else(|| {
                    let x = game
                        .registries
                        .blocks
                        .get_by_namespace("minecraft:air")
                        .expect("minecraft:air is missing");
                    warn!("Block {} not found, using air instead", layer.block);
                    x
                })
                .clone();
            layers.push(LoadedLayer {
                block,
                height: layer.height as i16,
            });
        }
        Self {
            settings,
            layers,
            game,
        }
    }

    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Self::Chunk {
        let mut chunk = AxolotlChunk::new(ChunkPos::new(chunk_x, chunk_z));
        self.generate_chunk_into(&mut chunk);
        return chunk;
    }

    fn generate_chunk_into(&self, chunk: &mut Self::Chunk) {
        for (y, layer) in self.layers.iter().enumerate() {
            for x in 0..16 {
                for z in 0..16 {
                    for y_v in 0..=layer.height {
                        let y = y as i16 + y_v;
                        chunk.set_block(
                            BlockPosition::new(x, y, z),
                            PlacedBlock::from(layer.block.clone()),
                        );
                    }
                }
            }
        }
        let air = self
            .game
            .registries
            .blocks
            .get_by_namespace("minecraft:air")
            .expect("minecraft:air is missing");

        for y in self.layers.len()..16 {
            for x in 0..16 {
                for z in 0..16 {
                    chunk.set_block(
                        BlockPosition::new(x, y as i16, z),
                        PlacedBlock::from(air.clone()),
                    );
                }
            }
        }
    }
}
