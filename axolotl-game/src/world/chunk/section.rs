use crate::world::chunk::biome_section::AxolotlBiomeSection;
use crate::world::chunk::blocks_section::AxolotlBlockSection;
use crate::world::chunk::consts::{SECTION_X_SIZE, SECTION_Y_SIZE, SECTION_Z_SIZE};
use axolotl_api::world::BlockPosition;
use axolotl_api::OwnedNameSpaceKey;
use axolotl_world::chunk::compact_array::CompactArrayIndex;
use bytemuck::{Pod, Zeroable};
use std::fmt::{Debug, Formatter};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Zeroable, Pod)]
#[repr(transparent)]
pub struct SectionPosIndex(u64);
impl CompactArrayIndex for SectionPosIndex {
    #[inline(always)]
    fn get(self) -> usize {
        self.0 as usize
    }
}
impl SectionPosIndex {
    /// Assumes the x,y,z are all in the range of 0-SECTION_{CORD}_SIZE
    #[inline(always)]
    pub fn from_block_pos_no_check(pos: BlockPosition) -> Self {
        Self::from((pos.x as u64, pos.y as u64, pos.z as u64))
    }
}
impl Debug for SectionPosIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = <SectionPosIndex as Into<(u64, u64, u64)>>::into(*self);
        write!(f, "SectionPosIndex(x: {}, y: {}, z: {})", x, y, z)
    }
}
impl<T: Into<u64>> From<(T, T, T)> for SectionPosIndex {
    #[inline(always)]
    fn from((x, y, z): (T, T, T)) -> Self {
        Self((y.into() << 8) | (z.into() << 4) | x.into())
    }
}
impl<T: From<u64>> Into<(T, T, T)> for SectionPosIndex {
    #[inline(always)]
    fn into(self) -> (T, T, T) {
        let value = self.0;
        let x = value & 0xF;
        let z = (value >> 4) & 0xF;
        let y = (value >> 8) & 0xF;
        (x.into(), y.into(), z.into())
    }
}
impl From<BlockPosition> for SectionPosIndex {
    fn from(pos: BlockPosition) -> Self {
        let x = ((pos.x.abs() as usize) % SECTION_X_SIZE) as u64;
        let y = ((pos.y.abs() as usize) % SECTION_Y_SIZE) as u64;
        let z = ((pos.z.abs() as usize) % SECTION_Z_SIZE) as u64;
        SectionPosIndex::from((x, y, z))
    }
}

#[derive(Debug, Error)]
pub enum InvalidChunkSection {
    #[error("Tried to set block out of bounds")]
    OutOfBounds,
    #[error("Invalid Block Data Num {0}")]
    InvalidBlock(i64),
}

#[derive(Debug, Clone)]
pub struct AxolotlChunkSection {
    pub blocks: AxolotlBlockSection,
    pub biomes: AxolotlBiomeSection,
    pub y: i8,
}
impl Default for AxolotlChunkSection {
    fn default() -> Self {
        AxolotlChunkSection::new(0)
    }
}
impl AxolotlChunkSection {
    pub fn new(y: i8) -> Self {
        Self {
            blocks: AxolotlBlockSection::default(),
            biomes: AxolotlBiomeSection::SingleBiome(OwnedNameSpaceKey::new(
                String::new(),
                String::new(),
            )),
            y,
        }
    }
}
