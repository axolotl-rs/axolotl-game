use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use uuid::Uuid;

use axolotl_api::item::block::{Block, BlockRules, BlockState};
use axolotl_api::world::{BlockPosition, World};

use crate::world::chunk::AxolotlChunk;
use crate::world::generator::AxolotlGenerator;

pub mod chunk;
mod generator;
pub mod level;
pub mod perlin;

#[derive(Debug)]
pub struct AxolotlWorld {
    pub uuid: Uuid,
    pub name: String,
    pub generator: Box<AxolotlGenerator>,
}

impl Hash for AxolotlWorld {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl World for AxolotlWorld {
    type Chunk = AxolotlChunk;
    type NoiseGenerator = AxolotlGenerator;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }

    fn generator(&self) -> &Self::NoiseGenerator {
        self.generator.as_ref()
    }

    fn set_block(
        &self,
        _location: impl Into<BlockPosition>,
        _block: impl Block<BlockState = impl BlockState, BlockRules = impl BlockRules>,
    ) {
        todo!()
    }
}
