use atomic_enum::atomic_enum;
use std::fmt::Debug;
use std::future::Future;
use std::ops::{Deref, DerefMut, Index};
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

use log::warn;
use minecraft_protocol::data::PacketDataType;
use parking_lot::RwLock;

use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_api::OwnedNameSpaceKey;
use axolotl_world::chunk::{ChunkSection, RawChunk};
use axolotl_world::entity::RawEntities;
use placed_block::PlacedBlock;

use crate::world::chunk::sections::Sections;
use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter};
use crate::AxolotlGame;

pub mod consts;
mod map;
pub mod network;
pub mod placed_block;
mod sections;

pub use map::ChunkMap;
#[derive(Debug)]
pub struct AxolotlChunk<W: World> {
    pub chunk_pos: ChunkPos,
    pub sections: Sections<W>,
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
        Self {
            chunk_pos,
            sections: Sections::default(),
        }
    }
    pub fn set_block(&mut self, mut pos: BlockPosition, block: PlacedBlock<W>) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set block out of bounds");
            return;
        }
        let section = &mut self.sections.as_mut()[id];
        section.blocks.set_block(pos, block);
    }
    pub fn set_biome(&mut self, mut pos: BlockPosition, biome: OwnedNameSpaceKey) {
        let id = pos.section();
        if id >= self.sections.len() {
            warn!("Tried to set biome out of bounds");
            return;
        }
        let section = &mut self.sections.as_mut()[id];
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
            let section = if raw_section.y_pos != self.sections.0[index].y {
                &mut self.sections.0[index]
            } else {
                // They should be in the same order BUT just in case
                if let Some(value) = self
                    .sections
                    .0
                    .iter_mut()
                    .find(|x| x.y == raw_section.y_pos)
                {
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
        let sections: Vec<ChunkSection> = self.sections.0.into_iter().map(|x| x.into()).collect();

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
#[derive(PartialEq, Eq, Hash, Default)]
#[repr(usize)]
#[atomic_enum(AtomicLoadState)]
pub enum LoadState {
    #[default]
    Unloaded = 0,
    Loading = 1,
    Loaded = 2,
    Unloading = 3,
}

#[derive(Debug)]
pub struct InnerChunkHandle<W: World> {
    pub value: RwLock<AxolotlChunk<W>>,
    pub loaded: AtomicLoadState,
}

pub struct ChunkFuture<W: World>(ChunkHandle<W>);
impl<W: World> Future for ChunkFuture<W> {
    type Output = Result<ChunkHandle<W>, ()>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.0.is_loaded() {
            std::task::Poll::Ready(Ok(self.0.clone()))
        } else {
            std::task::Poll::Pending
        }
    }
}
impl<W: World> InnerChunkHandle<W> {
    pub fn new(value: AxolotlChunk<W>) -> Self {
        Self {
            value: RwLock::new(value),
            loaded: AtomicLoadState::new(LoadState::Unloaded),
        }
    }
    pub fn wait_for_load(chunk: Arc<Self>) -> ChunkFuture<W> {
        ChunkFuture(chunk)
    }
    pub fn mark_loaded(&self) {
        self.loaded.store(LoadState::Loaded, Ordering::Relaxed);
    }
    pub fn mark_loading(&self) {
        self.loaded.store(LoadState::Loading, Ordering::Relaxed);
    }
    pub fn safe_to_load(&self) -> bool {
        self.loaded.load(Ordering::Relaxed) == LoadState::Unloaded
    }
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::Relaxed) == LoadState::Loaded
    }
}

pub type ChunkHandle<W> = Arc<InnerChunkHandle<W>>;
