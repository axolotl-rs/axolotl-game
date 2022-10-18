mod player;

use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter, RawChunk};
use crate::world::level::configs::WorldConfig;
use axolotl_api::world_gen::chunk::ChunkPos;
#[derive(Debug)]
pub struct Minecraft19WorldAccessor {}
impl LevelReader for Minecraft19WorldAccessor {
    type Error = crate::Error;

    fn get_chunk(&self, _chunk_pos: &ChunkPos) -> Result<Option<RawChunk>, Self::Error> {
        todo!("get_chunk")
    }
}
impl LevelWriter for Minecraft19WorldAccessor {
    type Error = crate::Error;

    fn save_chunk(&self, chunk_pos: ChunkPos, chunk: impl IntoRawChunk) -> Result<(), Self::Error> {
        todo!()
    }

    fn save_chunks(
        &self,
        _chunks: impl Iterator<Item = (ChunkPos, RawChunk)>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
