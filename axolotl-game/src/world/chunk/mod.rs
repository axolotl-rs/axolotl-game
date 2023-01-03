use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use log::{debug, info, warn};
use parking_lot::RwLock;
use tux_lockfree::map::{Map, Removed};
use tux_lockfree::queue::Queue;

use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_api::OwnedNameSpaceKey;
use axolotl_world::chunk::{ChunkSection, RawChunk};
use axolotl_world::entity::RawEntities;
use placed_block::PlacedBlock;

use crate::world::chunk::section::AxolotlChunkSection;
use crate::world::generator::AxolotlGenerator;
use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter};
use crate::world::ChunkUpdate;
use crate::{AxolotlGame, Error};

pub mod biome_section;
pub mod blocks_section;
pub mod consts;
pub mod placed_block;
pub mod section;

#[derive(Debug)]
pub struct AxolotlChunk<W: World> {
    pub chunk_pos: ChunkPos,
    pub sections: [AxolotlChunkSection<W>; (consts::Y_SIZE / consts::SECTION_Y_SIZE)],
}
impl<W: World> Clone for AxolotlChunk<W> {
    fn clone(&self) -> Self {
        Self {
            chunk_pos: self.chunk_pos,
            sections: self.sections.clone(),
        }
    }
}
impl<W: World> AxolotlChunk<W> {
    pub fn new(chunk_pos: ChunkPos) -> Self {
        let mut sections: [AxolotlChunkSection<W>; (consts::Y_SIZE / consts::SECTION_Y_SIZE)] =
            Default::default();
        for index in consts::MIN_Y_SECTION..consts::MAX_Y_SECTION {
            let section = &mut sections[(index + 4) as usize];
            section.y = index;
        }
        Self {
            chunk_pos,
            sections,
        }
    }
    pub fn set_block(&mut self, mut pos: BlockPosition, block: PlacedBlock<W>) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set block out of bounds");
            return;
        }
        let section = &mut self.sections[id];
        section.blocks.set_block(pos, block);
    }
    pub fn set_biome(&mut self, mut pos: BlockPosition, biome: OwnedNameSpaceKey) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set biome out of bounds");
            return;
        }
        let section = &mut self.sections[id];
        section.biomes.set_biome(pos, biome);
    }
}
impl<W: World> IntoRawChunk<W> for AxolotlChunk<W> {
    fn load_from_chunk(
        &mut self,
        game: Arc<AxolotlGame<W>>,
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
                if let Err(e) = section.blocks.load(game.as_ref(), blocks_section) {
                    warn!("Failed to load blocks section: {}", e);
                }
            } else {
                *section = Default::default();
            }

            if let Some(_biome_section) = raw_section.biomes.as_mut() {
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
            status: "full".to_string(),
            last_updated: 3912,
            inhabited_time: 0,
        }
    }
}
#[derive(Debug)]
pub struct ChunkHandle<W: World> {
    pub value: RwLock<AxolotlChunk<W>>,
    pub loaded: AtomicBool,
}
#[derive(Debug)]
pub struct ChunkMap<W: World, V: LevelReader<W> + LevelWriter<W> + Debug> {
    pub generator: AxolotlGenerator<W>,

    pub thread_safe_chunks: Map<ChunkPos, ChunkHandle<W>>,
    pub dead_chunks: Queue<AxolotlChunk<W>>,
    // Load Queue
    pub queue: Queue<ChunkUpdate<W>>,
    pub accessor: V,
}
impl<W: World, V: LevelReader<W> + LevelWriter<W> + Debug> ChunkMap<W, V>
where
    Error: From<<V as LevelWriter<W>>::Error> + From<<V as LevelReader<W>>::Error>,
{
    pub fn new(generator: AxolotlGenerator<W>, accessor: V) -> Self {
        Self {
            generator,
            thread_safe_chunks: Map::new(),
            dead_chunks: Queue::new(),
            queue: Queue::new(),
            accessor,
        }
    }

    /// Handles all updates within the queue

    #[deny(clippy::panic)]
    pub fn handle_updates(&self) {
        while let Some(update) = self.queue.pop() {
            if let Err(error) = self.handle_update(update) {
                warn!("Error handling chunk update: {:?}", error);
            }
        }
    }
    /// Handles a single update
    pub fn handle_update(&self, update: ChunkUpdate<W>) -> Result<(), Error> {
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
                    (*e.key(), (*value.value.read()).clone())
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
        update: Option<(BlockPosition, PlacedBlock<W>)>,
    ) -> Result<(), Error> {
        let pos = ChunkPos::new(x, z);
        info!("Loading chunk at {:?}", pos);
        let chunk = if let Some(dead) = self.dead_chunks.pop() {
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
        debug!("Loading chunk at {:?}", pos);
        if !self.accessor.get_chunk_into(&pos, chunk_ref)? {
            chunk_ref.chunk_pos = pos;
            debug!("Generating chunk at {:?}", pos);
            self.generator.generate_chunk_into(chunk_ref);
        }

        if let Some((pos, block)) = update {
            chunk_ref.set_block(pos, block);
        }
        drop(chunk);
        info!("Loaded chunk at {:?}", pos);
        map_guard
            .val()
            .loaded
            .store(true, std::sync::atomic::Ordering::Relaxed);

        Ok(())
    }
    pub fn save_all(&self) -> Result<(), Error> {
        let result: Vec<_> = self.thread_safe_chunks.iter().map(|x| *x.key()).collect();
        for pos in result {
            match self.unload_chunk(pos.0, pos.1) {
                Ok(ok) => {}
                Err(v) => {
                    warn!("Error saving chunk: {:?}", v);
                }
            }
        }
        Ok(())
    }
}
