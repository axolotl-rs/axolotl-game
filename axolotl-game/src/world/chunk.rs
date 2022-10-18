use ahash::AHashMap;
use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::world::BlockPosition;
use axolotl_api::NameSpaceRef;
use log::warn;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tux_lockfree::map::{Map, ReadGuard, Removed};
use tux_lockfree::queue::Queue;

use crate::world::block::MapState;
use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter};
use crate::world::ChunkUpdate;
use crate::{Error, MinecraftBlock};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_world::chunk::{ChunkSection, RawChunk};

#[derive(Debug, Clone)]
pub struct PlacedBlock {
    pub state: MapState,
    pub block: &'static MinecraftBlock,
}
impl PlacedBlock {
    pub fn id(&self) -> usize {
        self.block.id()
    }
}
impl Item for PlacedBlock {
    fn get_namespace(&self) -> NameSpaceRef<'static> {
        self.block.get_namespace()
    }
}

impl Block for PlacedBlock {
    type State = MapState;
    type PlacedBlock = Self;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        self.clone()
    }

    fn get_default_state(&self) -> Self::State {
        self.state.clone()
    }
}
#[derive(Debug, Clone, Default)]
pub struct AxolotlChunkSection {}
impl AxolotlChunkSection {
    pub fn set_block(&mut self, _pos: BlockPosition, _block: PlacedBlock) {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub struct AxolotlChunk {
    pub chunk_x: i64,
    pub chunk_z: i64,
    pub(crate) sections: [AxolotlChunkSection; 18],
}
impl AxolotlChunk {
    pub fn set_block(&mut self, mut pos: BlockPosition, block: PlacedBlock) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set block out of bounds");
            return;
        }
        let section = &mut self.sections[id];
        section.set_block(pos, block);
    }
}
impl IntoRawChunk for AxolotlChunk {
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
    // TODO make version of lockfree map that is meant for inner mutable types
    pub thread_safe_chunks: Map<ChunkPos, ChunkHandle>,
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
    pub fn unload_chunk(&self, x: i64, z: i64) -> Result<(), Error> {
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
        x: i64,
        z: i64,
        update: Option<(BlockPosition, PlacedBlock)>,
    ) -> Result<(), Error> {
        todo!("Load chunk");
    }
}
