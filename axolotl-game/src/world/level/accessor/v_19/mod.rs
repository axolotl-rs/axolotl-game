mod player;

use crate::world::level::accessor::{LevelReader, LevelWriter, RawChunk};
use crate::world::level::configs::WorldConfig;
use axolotl_api::world_gen::chunk::ChunkPos;

#[derive(Debug)]
pub enum LevelAccessError {}
pub struct Minecraft19WorldAccessor<'world> {
    pub world_config: &'world WorldConfig,
}
impl<'world> LevelReader for Minecraft19WorldAccessor<'world> {
    type Error = LevelAccessError;

    fn get_chunk(&self, _chunk_pos: &ChunkPos) -> Result<Option<RawChunk>, Self::Error> {
        todo!("get_chunk")
    }
}
impl<'world> LevelWriter for Minecraft19WorldAccessor<'world> {
    type Error = LevelAccessError;

    fn set_chunk(&mut self, _chunk_pos: ChunkPos, _chunk: RawChunk) -> Result<(), Self::Error> {
        todo!()
    }

    fn save_chunks(
        &mut self,
        _chunks: impl Iterator<Item = (ChunkPos, RawChunk)>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
