use axolotl_api::world::{BlockPosition, World};

use crate::world::chunk::placed_block::PlacedBlock;

pub mod chunk;
pub mod entity;
pub mod generator;
pub mod level;
pub mod perlin;
#[derive(Debug)]
pub enum ChunkUpdate<W: World> {
    Unload {
        x: i32,
        z: i32,
    },
    Load {
        x: i32,
        z: i32,
        set_block: Option<(BlockPosition, PlacedBlock<W>)>,
    },
}

impl<W: World> ChunkUpdate<W> {
    pub fn get_region(&self) -> (i32, i32) {
        match self {
            ChunkUpdate::Unload { x, z } => (*x >> 5, *z >> 5),
            ChunkUpdate::Load { x, z, .. } => (*x >> 5, *z >> 5),
        }
    }
}
