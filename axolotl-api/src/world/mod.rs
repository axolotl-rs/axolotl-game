use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use location::GenericLocation;
pub use location::Location;
pub use location::WorldLocation;

use crate::item::block::{Block, BlockState};
use crate::world_gen::chunk::ChunkPos;

mod location;

pub struct WorldGenerator {
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i16,
    pub z: i64,
}
impl BlockPosition {
    pub fn new(x: i64, y: i16, z: i64) -> Self {
        Self { x, y, z }
    }
    pub fn chunk(&mut self) -> ChunkPos {
        self.x %= 16;
        self.z %= 16;
        ChunkPos::new(self.x / 16, self.z / 16)
    }
}
impl<L: Location> From<L> for BlockPosition {
    fn from(l: L) -> Self {
        Self {
            x: l.get_x() as i64,
            y: l.get_y() as i16,
            z: l.get_z() as i64,
        }
    }
}

impl From<(i64, i16, i64)> for BlockPosition {
    fn from((x, y, z): (i64, i16, i64)) -> Self {
        Self { x, y, z }
    }
}

pub trait World: Send + Sync + Hash + Debug {
    type Chunk;
    type NoiseGenerator: for<'game> crate::world_gen::noise::ChunkGenerator<
        'game,
        Chunk = Self::Chunk,
    >;
    type WorldBlock: Block;
    fn get_name(&self) -> &str;

    fn uuid(&self) -> &Uuid;

    fn tick(&mut self);

    fn generator(&self) -> &Self::NoiseGenerator;

    fn set_block(&self, location: BlockPosition, block: Self::WorldBlock);
}
