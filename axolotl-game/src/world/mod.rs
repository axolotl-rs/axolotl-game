use ahash::AHashMap;
use dumbledore::entities::entity::{Entity, EntityLocation};
use log::warn;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc::Sender;
use tux_lockfree::queue::Queue;

use uuid::Uuid;

use crate::AxolotlGame;
use axolotl_api::item::block::BlockState;
use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;

use crate::world::chunk::{AxolotlChunk, PlacedBlock};
use crate::world::entity::player::{Chunks, GamePlayer};
use crate::world::entity::MinecraftEntity;
use crate::world::generator::AxolotlGenerator;
use crate::world::level::accessor::PlayerAccess;
use crate::world::level::configs::{WorldConfig, WorldGrouping};
use entity::player::PlayerUpdate;

pub mod block;
pub mod chunk;
pub mod entity;
pub mod generator;
pub mod level;
pub mod perlin;

pub trait WorldResourcePool {
    type PlayerAccess: PlayerAccess;
    fn world_group(&self) -> &WorldGrouping;

    fn player_access(&self) -> &Self::PlayerAccess;
}
pub struct SharedWorldResourcePool<'game> {
    pub worlds: Vec<AxolotlWorld<'game>>,
}
pub struct OwnedWorldResourcePool<'game> {
    pub world: AxolotlWorld<'game>,
}
pub enum GenericWorldResourcePool<'game> {
    Shared(SharedWorldResourcePool<'game>),
    Owned(OwnedWorldResourcePool<'game>),
}

#[derive(Debug)]
pub enum ChunkUpdate {
    Unload { x: i64, z: i64 },
    Load { x: i64, z: i64 },
}
impl ChunkUpdate {
    pub fn get_region(&self) -> (i64, i64) {
        match self {
            ChunkUpdate::Unload { x, z } => (*x >> 5, *z >> 5),
            ChunkUpdate::Load { x, z } => (*x >> 5, *z >> 5),
        }
    }
}
#[derive(Debug)]
pub struct AxolotlWorld<'game> {
    pub game: &'game AxolotlGame,
    pub uuid: Uuid,
    pub name: String,
    pub generator: Box<AxolotlGenerator<'game>>,
    pub chunks: HashMap<ChunkPos, AxolotlChunk>,
    pub world_config: WorldConfig,
    // TODO change type
    pub player_entities: AHashMap<(Entity, EntityLocation), Sender<PlayerUpdate>>,
    pub render_distance: u8,
    pub simulation_distance: u8,
    pub pending_new_players: Queue<(Entity, Sender<PlayerUpdate>)>,
    pub entities: Vec<MinecraftEntity>,
    pub chunk_updates: Queue<ChunkUpdate>,
    pub game_world: dumbledore::world::World,
}
impl<'game> AxolotlWorld<'game> {
    pub fn teleport_player_from_different_world(&self, player: (Entity, Sender<PlayerUpdate>)) {
        self.pending_new_players.push(player);
    }

    pub fn update_chunks(&mut self, chunk_requests: impl Iterator<Item = ChunkUpdate>) {
        let player_archetype = self
            .game_world
            .get_archetype::<GamePlayer>()
            .expect("GamePlayer archetype not found");
        for update in chunk_requests {
            match update {
                ChunkUpdate::Load { x, z } => {
                    // TODO load chunk
                    for ((_, loc), sender) in self.player_entities.iter() {
                        let tracked_chunks = player_archetype.get_comp::<Chunks>(loc.index);
                        if let Ok(Some(v)) = tracked_chunks {
                            if v.as_ref().0.contains(&(x, z)) {
                                if let Err(e) =
                                    sender.try_send(PlayerUpdate::LoadChunk { /* TODO data */ })
                                {
                                    warn!("Failed to send chunk update to player: {:?}", e);
                                }
                            }
                        }
                    }
                }
                ChunkUpdate::Unload { x, z } => {
                    for ((_, loc), sender) in self.player_entities.iter() {
                        let tracked_chunks = player_archetype.get_comp_mut::<Chunks>(loc.index);
                        if let Ok(Some(mut v)) = tracked_chunks {
                            if v.as_mut().0.remove(&(x, z)) {
                                if let Err(e) =
                                    sender.try_send(PlayerUpdate::UnloadChunk { /* TODO data */ })
                                {
                                    warn!("Failed to send chunk update to player: {:?}", e);
                                }
                            }
                        }
                    }
                    // TODO unload chunk + save
                }
            }
        }
    }
    pub fn tick_entities(&mut self) {}
}
impl Hash for AxolotlWorld<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<'game> World for AxolotlWorld<'game> {
    type Chunk = AxolotlChunk;
    type NoiseGenerator = AxolotlGenerator<'game>;
    type WorldBlock = PlacedBlock;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }

    fn tick(&mut self) {}

    fn generator(&self) -> &Self::NoiseGenerator {
        self.generator.as_ref()
    }

    fn set_block(&self, mut location: BlockPosition, _block: PlacedBlock) {
        let position = location.chunk();
        if let Some(_value) = self.chunks.get(&position) {
        } else {
        }
    }
}
