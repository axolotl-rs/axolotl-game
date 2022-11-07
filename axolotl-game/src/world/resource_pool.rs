use crate::world::chunk::ChunkMap;
use crate::world::level::accessor::v_19::player::Minecraft19PlayerAccess;
use crate::world::level::accessor::v_19::Minecraft19WorldAccessor;
use crate::world::level::configs::WorldGrouping;
use crate::world::AxolotlWorld;
use axolotl_api::world::World;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
pub type ChunkMapWrap = Arc<ChunkMap<Minecraft19WorldAccessor>>;

///
#[derive(Debug)]
pub struct SharedWorldResourcePool {
    pub player_access: Arc<Minecraft19PlayerAccess>,
    pub worlds: Vec<AxolotlWorld>,
    pub chunk_maps: Vec<ChunkMapWrap>,
    pub running: Arc<AtomicBool>,
}

impl SharedWorldResourcePool {
    pub fn tick(worlds: &mut [AxolotlWorld]) {
        // As of now this is more for representing the future
        for world in worlds.iter_mut() {
            world.tick();
        }
    }
    pub fn update_chunks(chunk_maps: Vec<ChunkMapWrap>) -> Vec<ChunkMapWrap> {
        for chunk_map in chunk_maps.iter() {
            chunk_map.handle_updates();
        }
        chunk_maps
    }
}
