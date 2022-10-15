pub mod v_19;

use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_world::chunk::RawChunk;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RegionLocation(pub i64, pub i64);
impl RegionLocation {
    pub fn from_chunk_pos(chunk_pos: &ChunkPos) -> Self {
        let (x, z) = chunk_pos.as_xz();
        Self::from_chunk_location(x, z)
    }
    #[inline(always)]
    pub fn from_chunk_location(x: i64, z: i64) -> Self {
        Self(x >> 5, z >> 5)
    }
    #[inline]
    pub fn format(&self) -> String {
        format!("r.{}.{}.mca", self.0, self.1)
    }
}
pub trait PlayerAccess {}

pub trait LevelReader {
    type Error: Debug;

    fn get_chunk(&self, chunk_pos: &ChunkPos) -> Result<Option<RawChunk>, Self::Error>;
}
pub trait LevelWriter {
    type Error: Debug;

    fn set_chunk(&mut self, chunk_pos: ChunkPos, chunk: RawChunk) -> Result<(), Self::Error>;
    fn save_chunks(
        &mut self,
        chunks: impl Iterator<Item = (ChunkPos, RawChunk)>,
    ) -> Result<(), Self::Error>;
}
