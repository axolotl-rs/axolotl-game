use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use location::GenericLocation;
pub use location::Location;
pub use location::WorldLocation;

use crate::item::block::{Block, BlockState};

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
    type NoiseGenerator: for<'game> crate::world_gen::noise::ChunkGenerator<
        'game,
        Chunk = Self::Chunk,
    >;
    type WorldBlock: Block;
    fn get_name(&self) -> &str;

    fn uuid(&self) -> &Uuid;

    fn generator(&self) -> &Self::NoiseGenerator;

    fn set_block(&self, location: BlockPosition, block: Self::WorldBlock);
}
