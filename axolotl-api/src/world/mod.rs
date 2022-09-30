use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use location::GenericLocation;
pub use location::Location;
pub use location::WorldLocation;

use crate::item::block::{Block, BlockRules, BlockState};

mod location;

pub struct WorldGenerator {
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl<L: Location> From<L> for BlockPosition {
    fn from(l: L) -> Self {
        Self {
            x: l.get_x() as i32,
            y: l.get_y() as i32,
            z: l.get_z() as i32,
        }
    }
}

impl From<(i32, i32, i32)> for BlockPosition {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

pub trait World: Send + Sync + Hash + Debug {
    type Chunk;
    type NoiseGenerator: crate::world_gen::noise::ChunkGenerator<Chunk = Self::Chunk>;

    fn get_name(&self) -> &str;

    fn uuid(&self) -> &Uuid;

    fn generator(&self) -> &Self::NoiseGenerator;

    fn set_block(
        &self,
        location: impl Into<BlockPosition>,
        block: impl Block<BlockState = impl BlockState, BlockRules = impl BlockRules>,
    );
}

impl<W: World> World for Box<W> {
    type Chunk = W::Chunk;
    type NoiseGenerator = W::NoiseGenerator;

    fn get_name(&self) -> &str {
        self.as_ref().get_name()
    }

    fn uuid(&self) -> &Uuid {
        self.as_ref().uuid()
    }

    fn generator(&self) -> &Self::NoiseGenerator {
        self.as_ref().generator()
    }

    fn set_block(
        &self,
        location: impl Into<BlockPosition>,
        block: impl Block<BlockState = impl BlockState, BlockRules = impl BlockRules>,
    ) {
        self.as_ref().set_block(location, block);
    }
}
