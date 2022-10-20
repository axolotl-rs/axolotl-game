use axolotl_api::world::BlockPosition;

use log::warn;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use tux_lockfree::map::{Map, Removed};
use tux_lockfree::queue::Queue;

use crate::world::chunk::section::AxolotlChunkSection;
use crate::world::generator::AxolotlGenerator;
use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter};
use crate::world::ChunkUpdate;
use crate::{AxolotlGame, Error};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_world::chunk::{ChunkSection, RawChunk};
use axolotl_world::entity::RawEntities;
use placed_block::PlacedBlock;

pub mod biome_section;
pub mod blocks_section;
pub mod consts;
pub mod placed_block;
pub mod section;

#[derive(Debug, Clone)]
pub struct AxolotlChunk<'game> {
    pub chunk_pos: ChunkPos,
    pub sections: [AxolotlChunkSection<'game>; (consts::Y_SIZE / consts::SECTION_Y_SIZE)],
}
impl<'game> AxolotlChunk<'game> {
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
    pub fn set_block(&mut self, mut pos: BlockPosition, block: PlacedBlock<'game>) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set block out of bounds");
            return;
        }
        let section = &mut self.sections[id];
        section.blocks.set_block(pos, block);
    }
}
impl<'game> IntoRawChunk<'game> for AxolotlChunk<'game> {
    fn load_from_chunk(
        &mut self,
        game: &'game AxolotlGame,
        chunk: &mut RawChunk,
        _entities: Option<&mut RawEntities>,
    ) {
        for (index, raw_section) in chunk.sections.iter_mut().enumerate() {
            let section = if raw_section.y_pos != self.sections[index].y {
                &mut self.sections[index]
            } else {
                // They should be in the same order BUT just in case
                if let Some(value) = self.sections.iter_mut().find(|x| x.y == raw_section.y_pos) {
                    value
                } else {
                    warn!("Tried to load chunk with invalid section");
                    continue;
                }
            };
            if let Some(blocks_section) = raw_section.block_states.as_mut() {
                if let Err(e) = section.blocks.load(game, blocks_section) {
                    warn!("Failed to load blocks section: {}", e);
                }
            } else {
                *section = Default::default();
            }

            if let Some(biome_section) = raw_section.biomes.as_mut() {
                warn!("Biome section not implemented");
            } else {
                *section = Default::default();
            }
        }
    }

    fn into_raw_chunk(self) -> RawChunk {
        let sections: Vec<ChunkSection> = self.sections.into_iter().map(|x| x.into()).collect();

        RawChunk {
            data_version: consts::DATA_VERSION,
            x_pos: self.chunk_pos.0,
            y_pos: -4,
            z_pos: self.chunk_pos.1,
            last_update: 0,
            sections,
            lights: vec![],
        }
    }
}
#[derive(Debug)]
pub struct ChunkHandle<'game> {
    pub value: RwLock<AxolotlChunk<'game>>,
    pub loaded: AtomicBool,
}
#[derive(Debug)]
pub struct ChunkMap<'game, V: LevelReader<'game> + LevelWriter<'game> + Debug> {
    pub generator: AxolotlGenerator<'game>,

    pub thread_safe_chunks: Map<ChunkPos, ChunkHandle<'game>>,
    pub dead_chunks: Queue<AxolotlChunk<'game>>,
    // Load Queue
    pub queue: Queue<ChunkUpdate<'game>>,
    pub accessor: V,
}
impl<'game, V: LevelReader<'game> + LevelWriter<'game> + Debug> ChunkMap<'game, V>
where
    Error: From<<V as LevelWriter<'game>>::Error> + From<<V as LevelReader<'game>>::Error>,
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
    pub fn handle_update(&self, update: ChunkUpdate<'game>) -> Result<(), Error> {
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
        update: Option<(BlockPosition, PlacedBlock<'game>)>,
    ) -> Result<(), Error> {
        let pos = ChunkPos::new(x, z);
        let mut chunk = if let Some(dead) = self.dead_chunks.pop() {
            dead
        } else {
            AxolotlChunk::new(pos)
        };
        let handle = ChunkHandle {
            value: RwLock::new(chunk),
            loaded: AtomicBool::new(false),
        };
        self.thread_safe_chunks.insert(pos, handle);
        let map_guard = self.thread_safe_chunks.get(&pos).unwrap();

        let mut chunk = map_guard.val().value.write();
        let chunk_ref = chunk.deref_mut();
        if !self.accessor.get_chunk_into(&pos, chunk_ref)? {
            chunk_ref.chunk_pos = pos;
            self.generator.generate_chunk_into(chunk_ref);
        }

        if let Some((pos, block)) = update {
            chunk_ref.set_block(pos, block);
        }
        drop(chunk);
        map_guard
            .val()
            .loaded
            .store(true, std::sync::atomic::Ordering::Relaxed);

        Ok(())
    }
}
