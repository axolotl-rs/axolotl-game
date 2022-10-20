use ahash::{AHashMap, AHashSet};
use dumbledore::entities::entity::{Entity, EntityLocation};
use log::{debug, warn};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tux_lockfree::queue::Queue;

use uuid::Uuid;

use axolotl_api::item::block::BlockState;
use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;

use crate::world::chunk::{AxolotlChunk, ChunkMap};
use crate::world::entity::player::GamePlayer;
use crate::world::entity::MinecraftEntity;
use crate::world::generator::AxolotlGenerator;
use crate::world::level::configs::WorldConfig;
use axolotl_world::entity::player::PlayerData;
use chunk::placed_block::PlacedBlock;
use dumbledore::world::World as ECSWorld;
use entity::player::PlayerUpdate;

pub mod block;
pub mod chunk;
pub mod entity;
pub mod generator;
pub mod level;
pub mod perlin;
mod resource_pool;

use crate::world::entity::properties::Location;
use crate::world::level::accessor::v_19::player::Minecraft19PlayerAccess;
use crate::world::level::accessor::v_19::Minecraft19WorldAccessor;
use crate::Sender;

#[derive(Debug)]
pub enum ChunkUpdate<'game> {
    Unload {
        x: i32,
        z: i32,
    },
    Load {
        x: i32,
        z: i32,
        set_block: Option<(BlockPosition, PlacedBlock<'game>)>,
    },
}
impl ChunkUpdate<'_> {
    pub fn get_region(&self) -> (i32, i32) {
        match self {
            ChunkUpdate::Unload { x, z } => (*x >> 5, *z >> 5),
            ChunkUpdate::Load { x, z, .. } => (*x >> 5, *z >> 5),
        }
    }
}
#[derive(Debug)]
pub struct WorldPlayer {
    pub location: EntityLocation,
    pub sender: Sender<Arc<PlayerUpdate>>,
}
impl Hash for WorldPlayer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.index.hash(state);
    }
}
#[derive(Debug, Clone)]
pub struct ChunkTickets {
    pub tickets: AHashMap<ChunkPos, AHashSet<Entity>>,
}
impl ChunkTickets {
    pub fn find_chunks_to_unload<UC>(&mut self, unload_chunk: UC)
    where
        UC: Fn(ChunkPos),
    {
        for (pos, tickets) in self.tickets.iter() {
            if tickets.is_empty() {
                unload_chunk(*pos);
            }
        }
    }
}
#[derive(Debug)]
pub struct AxolotlWorld<'game> {
    pub uuid: Uuid,
    pub name: String,
    pub world_config: WorldConfig,
    pub clients: AHashMap<Entity, WorldPlayer>,
    pub render_distance: u8,
    pub simulation_distance: u8,
    pub entities: Vec<MinecraftEntity>,
    pub game_world: ECSWorld,
    pub chunk_map: Arc<ChunkMap<'game, Minecraft19WorldAccessor<'game>>>,
    pub chunk_tickets: ChunkTickets,
    pub new_players: Queue<(Entity, WorldPlayer)>,
    pub player_access: Arc<Minecraft19PlayerAccess>,
}
impl<'game> AxolotlWorld<'game> {
    pub fn load_player(&self, player: Sender<Arc<PlayerUpdate>>, nbt: PlayerData) {
        let game_player = GamePlayer::from(nbt);
        let (entity, location) = self
            .game_world
            .add_entity(game_player)
            .expect("Failed to add player entity");
        self.new_players.push((
            entity,
            WorldPlayer {
                location,
                sender: player,
            },
        ));
    }
    /// Location is a specified via a teleport
    /// If None we will assume it used a portal and will fall back on Game Logic
    pub fn load_player_from_tp(
        &self,
        _player: Sender<Arc<PlayerUpdate>>,
        _game_player: GamePlayer,
        _location: Option<Location>,
    ) {
    }

    pub(crate) fn send_block_update(&self, pos: BlockPosition, block: usize) {
        let chunk_x = pos.x as i32 / 16;
        let chunk_z = pos.z as i32 / 16;
        let pos1 = ChunkPos::new(chunk_x, chunk_z);
        let update = Arc::new(PlayerUpdate::UpdateBlock(pos, block));

        self.push_update_to_players_at(pos1, update);
    }
    pub(crate) fn send_block_updates(
        &self,
        chunk: ChunkPos,
        blocks: impl Iterator<Item = (BlockPosition, usize)>,
    ) {
        let mut section_updates: AHashMap<i64, Vec<i64>> = AHashMap::with_capacity(16);
        let (chunk_x, chunk_y): (i32, i32) = chunk.into();
        let chunk_x = chunk_x as i64;
        let chunk_y = chunk_y as i64;
        for (pos, id) in blocks {
            let id = id as i64;
            let section_pos =
                (chunk_x & 0x3FFFFF) << 42 | (pos.y as i64 & 0xFFFFF) | (chunk_y & 0x3FFFFF) << 20;
            let block_pos = (id << 12) | pos.x << 8 | pos.z << 4 | (pos.y as i64 & 0xF);

            if let Some(section) = section_updates.get_mut(&section_pos) {
                section.push(block_pos)
            } else {
                section_updates.insert(section_pos, vec![block_pos]);
            }
        }
        let update = Arc::new(PlayerUpdate::SectionUpdate(section_updates));
        self.push_update_to_players_at(chunk, update);
    }
    pub fn push_update_to_players_at(&self, chunk: ChunkPos, update: Arc<PlayerUpdate>) {
        if let Some(entities) = self.chunk_tickets.tickets.get(&chunk) {
            for player in entities {
                if let Some(player) = self.clients.get(player) {
                    if let Err(error) = player.sender.send(update.clone()) {
                        warn!("Failed to send chunk update to player: {}", error);
                    }
                }
                // In theory this could happen if a player is being removed as we are iterating over the tracked chunks
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
    type Chunk = AxolotlChunk<'game>;
    type NoiseGenerator = AxolotlGenerator<'game>;
    type WorldBlock = PlacedBlock<'game>;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }

    fn tick(&mut self) {}

    fn generator(&self) -> &Self::NoiseGenerator {
        &self.chunk_map.generator
    }

    fn set_block(&self, location: BlockPosition, block: PlacedBlock<'game>) {
        let mut relative_pos = location.clone();
        let position = (relative_pos).chunk();
        let id = block.id();

        if let Some(value) = self.chunk_map.thread_safe_chunks.get(&position) {
            let mut guard = value.val().value.write();
            guard.set_block(relative_pos, block);
            drop(guard);
            drop(value);
            self.send_block_update(location, id);
        } else {
            debug!("Chunk not loading. Will load chunk and set block");
            self.chunk_map.queue.push(ChunkUpdate::Load {
                x: position.x(),
                z: position.z(),
                set_block: Some((location, block)),
            });
        }
    }

    fn set_blocks(
        &self,
        chunk_pos: ChunkPos,
        blocks: impl Iterator<Item = (BlockPosition, Self::WorldBlock)>,
    ) {
        let option = self.chunk_map.thread_safe_chunks.get(&chunk_pos);
        if let Some(value) = option {
            let mut block_len = Vec::with_capacity(blocks.size_hint().0);
            let mut guard = value.val().value.write();
            for (pos, block) in blocks {
                block_len.push((pos.clone(), block.id()));
                guard.set_block(pos, block);
            }
            drop(guard);
            drop(value);
            self.send_block_updates(chunk_pos, block_len.into_iter());
        } else {
            warn!("Attempted to set a group of blocks to an unloaded chunk");
        }
    }
}
