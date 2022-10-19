use axolotl_api::world::BlockPosition;

use log::warn;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use tux_lockfree::map::{Map, Removed};
use tux_lockfree::queue::Queue;

use crate::world::chunk::section::AxolotlChunkSection;
use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter};
use crate::world::ChunkUpdate;
use crate::Error;
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_world::chunk::RawChunk;
use axolotl_world::entity::RawEntities;
use placed_block::PlacedBlock;

pub mod biome_section;
mod blocks_section;
pub mod consts;
pub mod placed_block;
pub mod section;

#[derive(Debug, Clone)]
pub struct AxolotlChunk {
    pub chunk_pos: ChunkPos,
    pub sections: [AxolotlChunkSection; (consts::Y_SIZE / consts::SECTION_Y_SIZE)],
}
impl AxolotlChunk {
    pub fn new(chunk_pos: ChunkPos) -> Self {
        let mut sections: [AxolotlChunkSection; (consts::Y_SIZE / consts::SECTION_Y_SIZE)] =
            Default::default();
        for index in consts::MIN_Y_SECTION..consts::MAX_Y_SECTION {
            let section = &mut sections[index as usize + 4];
            section.y = index;
        }
        Self {
            chunk_pos,
            sections,
        }
    }
    pub fn set_block(&mut self, mut pos: BlockPosition, block: PlacedBlock) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set block out of bounds");
            return;
        }
        let section = &mut self.sections[id];
        section.blocks.set_block(pos, block);
    }
}
impl IntoRawChunk for AxolotlChunk {
    fn load_from_chunk(&mut self, _chunk: &mut RawChunk, _entities: Option<&mut RawEntities>) {
        todo!()
    }

    fn into_raw_chunk(self) -> RawChunk {
        todo!("into_raw_chunk")
    }
}
#[derive(Debug)]
pub struct ChunkHandle {
    pub value: RwLock<AxolotlChunk>,
    pub loaded: AtomicBool,
}
#[derive(Debug)]
pub struct ChunkMap<V: LevelReader + LevelWriter + Debug> {
    pub thread_safe_chunks: Map<ChunkPos, ChunkHandle>,
    pub dead_chunks: Queue<AxolotlChunk>,
    // Load Queue
    pub queue: Queue<ChunkUpdate>,
    pub accessor: V,
}
impl<V: LevelReader + LevelWriter + Debug> ChunkMap<V>
where
    Error: From<<V as LevelWriter>::Error> + From<<V as LevelReader>::Error>,
{
    /// Handles all updates within the queue
    pub fn handle_updates(&self) {
        while let Some(update) = self.queue.pop() {
            if let Err(error) = self.handle_update(update) {
                warn!("Error handling chunk update: {:?}", error);
            }
        }
    }
    /// Handles a single update
    pub fn handle_update(&self, update: ChunkUpdate) -> Result<(), Error> {
        match update {
            ChunkUpdate::Load { x, z, set_block } => {
                self.load_chunk(x, z, set_block)?;
            }
            ChunkUpdate::Unload { x, z } => {
                self.unload_chunk(x, z)?;
            }
        }
        Ok(())
    }
    #[inline(always)]
    pub fn unload_chunk(&self, x: i32, z: i32) -> Result<(), Error> {
        if let Some(value) = self.thread_safe_chunks.remove(&ChunkPos::new(x, z)) {
            let (pos, chunk) = match Removed::try_into(value) {
                Ok((pos, v)) => {
                    // NO need to update load because no other copies exist
                    (pos, v.value.into_inner())
                }
                Err(e) => {
                    let value = e.val();

                    // Update loaded because it has copies
                    value
                        .loaded
                        .store(false, std::sync::atomic::Ordering::Relaxed);
                    // Take a current snapshot of the chunk and save it. This is a bit of a hack
                    (e.key().clone(), (*value.value.read()).clone())
                }
            };
            self.accessor.save_chunk(pos, chunk)?;
        };
        Ok(())
    }
    /// Will run the update before putting chunk in map
    #[inline(always)]
    pub fn load_chunk(
        &self,
        x: i32,
        z: i32,
        update: Option<(BlockPosition, PlacedBlock)>,
    ) -> Result<(), Error> {
        let mut chunk = if let Some(dead) = self.dead_chunks.pop() {
            dead
        } else {
            AxolotlChunk::new(ChunkPos::new(x, z))
        };
        let pos = ChunkPos(x, z);
        self.accessor.get_chunk_into(&pos, &mut chunk)?;
        if let Some((pos, block)) = update {
            chunk.set_block(pos, block);
        }
        let handle = ChunkHandle {
            value: RwLock::new(chunk),
            loaded: AtomicBool::new(true),
        };
        self.thread_safe_chunks.insert(pos, handle);
        Ok(())
    }
}
