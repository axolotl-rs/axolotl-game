use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use ahash::AHashMap;
use log::{debug, info, warn};
use parking_lot::{Mutex, RwLock};

use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;

use crate::world::chunk::placed_block::PlacedBlock;
use crate::world::chunk::{AxolotlChunk, ChunkHandle, InnerChunkHandle, LoadState};
use crate::world::generator::AxolotlGenerator;
use crate::world::level::accessor::{LevelReader, LevelWriter};
use crate::world::ChunkUpdate;
use crate::Error;

type Queue<T> = Mutex<VecDeque<T>>;
type ThreadSafeChunks<W> = RwLock<AHashMap<ChunkPos, ChunkHandle<W>>>;

#[derive(Debug)]
pub struct ChunkMap<W: World, V: LevelReader<W> + LevelWriter<W> + Debug> {
    pub generator: AxolotlGenerator<W>,
    pub thread_safe_chunks: ThreadSafeChunks<W>,
    pub dead_chunks: Queue<AxolotlChunk<W>>,
    pub load_queue: Queue<ChunkUpdate<W>>,
    pub accessor: V,
}

impl<W: World, V: LevelReader<W> + LevelWriter<W> + Debug> ChunkMap<W, V>
where
    Error: From<<V as LevelWriter<W>>::Error> + From<<V as LevelReader<W>>::Error>,
{
    pub fn new(generator: AxolotlGenerator<W>, accessor: V) -> Self {
        Self {
            generator,
            thread_safe_chunks: ThreadSafeChunks::default(),
            dead_chunks: Queue::default(),
            load_queue: Queue::default(),
            accessor,
        }
    }
    #[inline]
    pub fn push_chunk_update(&self, update: ChunkUpdate<W>) {
        self.load_queue.lock().push_back(update);
    }

    /// Handles all updates within the queue

    #[deny(clippy::panic)]
    pub fn handle_updates(&self) {
        let mut guard = self.load_queue.lock();
        let queue = mem::take(guard.deref_mut());
        for update in queue {
            if let Err(error) = self.handle_update(update) {
                warn!("Error handling chunk update: {:?}", error);
            }
        }
    }
    /// Handles a single update
    pub fn handle_update(&self, update: ChunkUpdate<W>) -> Result<(), Error> {
        match update {
            ChunkUpdate::Load { x, z, set_block } => {
                self.load_chunk_task(x, z, set_block)?;
            }
            ChunkUpdate::Unload { x, z } => {
                self.unload_chunk(x, z)?;
            }
        }
        Ok(())
    }
    #[inline(always)]
    pub fn unload_chunk(&self, x: i32, z: i32) -> Result<(), Error> {
        let chunk_pos = ChunkPos::new(x, z);

        let mut pos = self.thread_safe_chunks.write();
        let removed = pos.remove(&chunk_pos);
        drop(pos);
        if let Some(value) = removed {
            self.unload_inner(chunk_pos, value)?;
        };
        Ok(())
    }
    /// Attempt to either get the inner value or clone it
    /// if the data is cloned it is marked as unloaded meaning any other handles know that it is unloaded
    #[inline(always)]
    fn unload_inner(
        &self,
        chunk_pos: ChunkPos,
        value: Arc<InnerChunkHandle<W>>,
    ) -> Result<(), Error> {
        let chunk = match Arc::try_unwrap(value) {
            Ok(chunk) => chunk.value.into_inner(),
            Err(e) => {
                // Marks the thread as unloaded and then clones the inner value
                e.loaded
                    .store(LoadState::Unloading, std::sync::atomic::Ordering::Relaxed);
                let guard = e.value.read();
                (guard.deref().clone())
            }
        };
        self.accessor.save_chunk(chunk_pos, chunk)?;
        Ok(())
    }
    /// Will run the update before putting chunk in map
    #[inline(always)]
    pub fn load_chunk_task(
        &self,
        x: i32,
        z: i32,
        update: Option<(BlockPosition, PlacedBlock<W>)>,
    ) -> Result<(), Error> {
        let pos = ChunkPos::new(x, z);
        info!("Loading chunk at {:?}", pos);
        let mut lock = self.thread_safe_chunks.write();
        info!("Got lock");
        let handle = if let Some(chunk) = lock.get(&pos) {
            info!("Chunk handle already exists");
            if !chunk.safe_to_load() {
                return Ok(());
            }
            chunk.clone()
        } else {
            info!("Creating new chunk at {:?}", pos);
            let handle = Self::create_chunk(&self.dead_chunks, pos);
            lock.insert(pos, handle.clone());
            drop(lock);
            handle
        };
        info!("Loading chunk at {:?} with handle {:?}", pos, handle);
        handle.mark_loading();
        let mut chunk = handle.value.write();
        info!("Get write lock for chunk at {:?}", pos);
        let chunk_ref = chunk.deref_mut();
        if !self.accessor.get_chunk_into(&pos, chunk_ref)? {
            chunk_ref.chunk_pos = pos;
            debug!("Generating chunk at {:?}", pos);
            self.generator.generate_chunk_into(chunk_ref);
        }

        if let Some((pos, block)) = update {
            chunk_ref.set_block(pos, block);
        }
        drop(chunk);

        handle.mark_loaded();
        info!("Loaded chunk at {:?}", pos);

        Ok(())
    }
    fn create_chunk(dead_chunks: &Queue<AxolotlChunk<W>>, pos: ChunkPos) -> ChunkHandle<W> {
        let mut dead_chunks = dead_chunks.lock();
        let chunk = if let Some(mut dead) = dead_chunks.pop_front() {
            dead.chunk_pos = pos;
            dead
        } else {
            AxolotlChunk::new(pos)
        };

        InnerChunkHandle::new(chunk).into()
    }
    // TODO How should errors be handled?
    pub fn save_all(&self) -> Result<(), Error> {
        let mut lock = self.thread_safe_chunks.write();

        for (chunk_pos, data) in lock.drain() {
            if let Err(e) = self.unload_inner(chunk_pos, data) {
                warn!("Error saving chunk: {:?}", e);
            }
        }
        Ok(())
    }

    pub fn load_chunk(&self, handle: ChunkHandle<W>) -> Result<(), Error> {
        handle.mark_loading();
        let mut chunk = handle.value.write();
        let chunk_ref = chunk.deref_mut();
        debug!("Loading chunk at {:?}", chunk_ref.chunk_pos);
        if !self
            .accessor
            .get_chunk_into(&chunk_ref.chunk_pos.clone(), chunk_ref)?
        {
            self.generator.generate_chunk_into(chunk_ref);
        }
        drop(chunk);

        Ok(())
    }
    /// Will return a ChunkHandle this may or may not be loaded
    pub fn get_chunk(&self, pos: ChunkPos) -> ChunkHandle<W> {
        let lock = self.thread_safe_chunks.read();
        if let Some(chunk) = lock.get(&pos) {
            chunk.clone()
        } else {
            drop(lock);
            let mut lock = self.thread_safe_chunks.write();
            let handle = Self::create_chunk(&self.dead_chunks, pos);
            lock.insert(pos, handle.clone());
            handle
        }
    }
}
