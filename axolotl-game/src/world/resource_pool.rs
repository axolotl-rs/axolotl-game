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

#[derive(Debug)]
pub struct SharedWorldResourcePool {
    pub player_access: Arc<Minecraft19PlayerAccess>,
    pub worlds: Vec<AxolotlWorld>,
    pub chunk_maps: Vec<Arc<ChunkMap<Minecraft19WorldAccessor>>>,
    pub running: Arc<AtomicBool>,
}

impl SharedWorldResourcePool {
    pub fn tick(mut self) {
        // TODO add timing
        while self.running.load(Ordering::Relaxed) {
            for world in self.worlds.iter_mut() {
                world.tick();
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
pub struct OwnedWorldResourcePool {
    pub world_group: WorldGrouping,
    pub world: Arc<RwLock<AxolotlWorld>>,
    pub chunk_map: Arc<ChunkMap<Minecraft19WorldAccessor>>,
    pub running: Arc<AtomicBool>,
}

impl OwnedWorldResourcePool {
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

pub enum GenericWorldResourcePool {
    Shared(SharedWorldResourcePool),
    Owned(OwnedWorldResourcePool),
}
impl GenericWorldResourcePool {
    pub fn run(mut self) {}
}
