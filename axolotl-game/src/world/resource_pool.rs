use crate::world::chunk::ChunkMap;
use crate::world::level::accessor::v_19::player::Minecraft19PlayerAccess;
use crate::world::level::accessor::v_19::Minecraft19WorldAccessor;
use crate::world::level::configs::WorldGrouping;
use crate::world::AxolotlWorld;
use axolotl_api::world::World;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub trait WorldResourcePool {
    fn world_group(&self) -> &WorldGrouping;
}

#[derive(Debug, Clone)]
pub struct SharedWorldResourcePool<'game> {
    pub player_access: Arc<Minecraft19PlayerAccess>,
    pub world_group: Arc<WorldGrouping>,
    pub worlds: Vec<Arc<RwLock<AxolotlWorld<'game>>>>,
    pub chunk_maps: Vec<Arc<ChunkMap<'game, Minecraft19WorldAccessor>>>,
    pub running: Arc<AtomicBool>,
}

impl SharedWorldResourcePool<'_> {
    pub fn tick(&self) {
        // TODO add timing
        while self.running.load(Ordering::Relaxed) {
            for world in &self.worlds {
                world.write().tick();
            }
        }
    }
    pub fn update_chunks(&self) {
        for chunk_map in &self.chunk_maps {
            chunk_map.handle_updates();
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedWorldResourcePool<'game> {
    pub world_group: WorldGrouping,
    pub world: Arc<RwLock<AxolotlWorld<'game>>>,
    pub chunk_map: Arc<ChunkMap<'game, Minecraft19WorldAccessor>>,
    pub running: Arc<AtomicBool>,
}

impl OwnedWorldResourcePool<'_> {
    pub fn tick(&self) {
        // TODO add timing
        while self.running.load(Ordering::Relaxed) {
            self.world.write().tick();
        }
    }
    pub fn update_chunks(&self) {
        self.chunk_map.handle_updates();
    }
}

pub enum GenericWorldResourcePool<'game> {
    Shared(SharedWorldResourcePool<'game>),
    Owned(OwnedWorldResourcePool<'game>),
}
