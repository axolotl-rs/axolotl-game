use std::fmt::Debug;
use std::sync::Arc;

use axolotl_api::world::World;
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_world::chunk::RawChunk;
use axolotl_world::entity::RawEntities;

use crate::AxolotlGame;

pub mod v_19;

#[derive(Debug)]
pub struct RegionLocation(pub i64, pub i64);
impl RegionLocation {
    pub fn from_chunk_pos(chunk_pos: &ChunkPos) -> Self {
        let (x, z): (i32, i32) = chunk_pos.into();
        Self::from_chunk_location(x as i64, z as i64)
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

pub trait LevelReader<W: World> {
    type Error: Debug;
    fn get_chunk_into(
        &self,
        chunk_pos: &ChunkPos,
        chunk: &mut impl IntoRawChunk<W>,
    ) -> Result<bool, Self::Error>;

    fn get_chunk(&self, chunk_pos: &ChunkPos) -> Result<Option<RawChunk>, Self::Error>;
}
pub trait LevelWriter<W: World> {
    type Error: Debug + Into<crate::Error>;

    fn save_chunk(
        &self,
        chunk_pos: ChunkPos,
        chunk: impl IntoRawChunk<W>,
    ) -> Result<(), Self::Error>;
    fn save_chunks(
        &self,
        chunks: impl Iterator<Item = (ChunkPos, RawChunk)>,
    ) -> Result<(), Self::Error>;
}

pub trait IntoRawChunk<W: World> {
    fn load_from_chunk(
        &mut self,
        game: Arc<AxolotlGame<W>>,
        chunk: &mut RawChunk,
        entities: Option<&mut RawEntities>,
    );

    fn into_raw_chunk(self) -> RawChunk;

    fn into_raw_chunk_use(self, chunk: &mut RawChunk)
    where
        Self: Sized,
    {
        *chunk = self.into_raw_chunk();
    }
}
