use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use uuid::Uuid;

use axolotl_api::item::block::{Block, BlockState};
use axolotl_api::world::{BlockPosition, World};

use crate::world::chunk::{AxolotlChunk, PlacedBlock};
use crate::world::generator::AxolotlGenerator;

pub mod block;
pub mod chunk;
pub mod generator;
pub mod level;
pub mod perlin;

#[derive(Debug)]
pub struct AxolotlWorld<'game> {
    pub uuid: Uuid,
    pub name: String,
    pub generator: Box<AxolotlGenerator<'game>>,
    phantom: std::marker::PhantomData<&'game ()>,
}

impl Hash for AxolotlWorld<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<'game> World for AxolotlWorld<'game> {
    type Chunk = AxolotlChunk;
    type NoiseGenerator = AxolotlGenerator<'game>;
    type WorldBlock = PlacedBlock;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }

    fn generator(&self) -> &Self::NoiseGenerator {
        self.generator.as_ref()
    }

    fn set_block(&self, location: BlockPosition, block: PlacedBlock) {
        todo!()
    }
}
