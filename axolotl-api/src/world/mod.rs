use bytemuck::{Pod, Zeroable};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Zeroable)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i16,
    pub z: i64,
}
impl BlockPosition {
    pub fn new(x: i64, y: i16, z: i64) -> Self {
        Self { x, y, z }
    }
    pub fn absolute(&self) -> Self {
        Self {
            x: self.x * 16,
            y: self.y,
            z: self.z * 16,
        }
    }
    pub fn absolute_ref(&mut self) {
        self.x *= 16;
        self.z *= 16;
    }
    pub fn make_relative_ref(&mut self) {
        self.x = self.x % 16;
        self.z = self.z % 16;
    }
    #[inline(always)]
    pub fn section(&mut self) -> usize {
        let section_index = ((self.y as usize) / 16);
        self.y %= 16;
        section_index
    }
    /// Returns the chunk position of the chunk this block is in
    /// Makes the x.y relative to the chunk
    #[inline(always)]
    pub fn chunk(&mut self) -> ChunkPos {
        let x = (self.x / 16);
        let z = (self.z / 16);
        self.x %= 16;
        self.z %= 16;
        ChunkPos::new(x as i32, z as i32)
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
    type WorldBlock;
    type NoiseGenerator: crate::world_gen::noise::ChunkGenerator<Chunk = Self::Chunk>;
    fn get_name(&self) -> &str;

    fn tick(&mut self);

    fn generator(&self) -> &Self::NoiseGenerator;

    fn set_block(
        &self,
        location: BlockPosition,
        block: Self::WorldBlock,
        require_loaded: bool,
    ) -> bool;
    ///
    /// Rules for the group set chunk
    /// 1. They must all be in the same chunk
    /// 2. The chunk must be loaded
    /// 3. BlockPos must already be relative to the chunk
    fn set_blocks(
        &self,
        chunk_pos: ChunkPos,
        blocks: impl Iterator<Item = (BlockPosition, Self::WorldBlock)>,
    );
}
