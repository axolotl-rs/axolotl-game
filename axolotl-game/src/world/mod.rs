use ahash::{AHashMap, AHashSet};
use axolotl_nbt::value::Value;
use log::{debug, warn};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use tux_lockfree::queue::Queue;

use uuid::Uuid;

use axolotl_api::item::block::BlockState;
use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;

use crate::world::chunk::{AxolotlChunk, ChunkMap};
use crate::world::entity::player::GamePlayer;
use crate::world::entity::MinecraftEntity;
use crate::world::generator::{AxolotlGenerator, ChunkSettings};
use crate::world::level::configs::WorldConfig;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_api::OwnedNameSpaceKey;
use axolotl_world::entity::player::PlayerData;
use axolotl_world::level::{Dimension, WorldGenSettings};
use chunk::placed_block::PlacedBlock;
use entity::player::PlayerUpdate;
use hecs::{Entity, World as ECSWorld};
use parking_lot::Mutex;
use serde_json::json;

pub mod chunk;
pub mod entity;
pub mod generator;
pub mod level;
pub mod perlin;
pub mod resource_pool;

use crate::world::entity::properties::Location;
use crate::world::level::accessor::v_19::player::Minecraft19PlayerAccess;
use crate::world::level::accessor::v_19::Minecraft19WorldAccessor;
use crate::{AxolotlGame, Error, Sender};

#[derive(Debug)]
pub enum ChunkUpdate {
    Unload {
        x: i32,
        z: i32,
    },
    Load {
        x: i32,
        z: i32,
        set_block: Option<(BlockPosition, PlacedBlock)>,
    },
}

impl ChunkUpdate {
    pub fn get_region(&self) -> (i32, i32) {
        match self {
            ChunkUpdate::Unload { x, z } => (*x >> 5, *z >> 5),
            ChunkUpdate::Load { x, z, .. } => (*x >> 5, *z >> 5),
        }
    }
}
pub type PlayerLocation = Arc<Mutex<Location>>;
#[derive(Debug)]
pub struct WorldPlayer {
    pub entity: Entity,
    pub uuid: Uuid,
    pub sender: Sender<Arc<PlayerUpdate>>,
    pub player_location: PlayerLocation,
}

impl PartialEq for WorldPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid && self.entity == other.entity
    }
}

impl Eq for WorldPlayer {}

impl Hash for WorldPlayer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChunkTickets {
    pub tickets: AHashMap<ChunkPos, AHashSet<Arc<WorldPlayer>>>,
}

impl ChunkTickets {
    pub fn remove_player(&mut self, player: &Arc<WorldPlayer>) {
        for (_, players) in self.tickets.iter_mut() {
            players.remove(player);
        }
    }
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
pub enum ServerUpdateIn {
    // A player has joined the server
    NewPlayer {
        sender: Sender<Arc<PlayerUpdate>>,
        uuid: Uuid,
    },
    PlayerLeft(Uuid),
    // To unload the chunk of entities
    UnloadChunk {
        x: i32,
        z: i32,
    },
}

#[derive(Debug)]
pub enum ServerUpdateOut {}

#[derive(Debug)]
pub struct WorldLoad {
    pub world: AxolotlWorld,
    // Updates from the server to the world
    pub sender: crate::Sender<ServerUpdateIn>,
    // Updates from the world to the server
    pub receiver: crate::Receiver<ServerUpdateOut>,
}

#[derive(Debug)]
pub struct InternalWorldRef {
    // Updates from the server to the world
    pub sender: crate::Sender<ServerUpdateIn>,
    // Updates from the world to the server
    pub receiver: crate::Receiver<ServerUpdateOut>,
}

impl InternalWorldRef {
    fn sender(&self) -> &crate::Sender<ServerUpdateIn> {
        &self.sender
    }
    fn receiver(&self) -> &crate::Receiver<ServerUpdateOut> {
        &self.receiver
    }
}

pub struct AxolotlWorld {
    pub full_name: String,
    pub name: String,
    pub name_hash: u64,
    pub clients: Vec<Arc<WorldPlayer>>,
    pub render_distance: u8,
    pub simulation_distance: u8,
    pub game_world: ECSWorld,
    pub chunk_map: Arc<ChunkMap<Minecraft19WorldAccessor>>,
    pub chunk_tickets: ChunkTickets,
    pub server_update_receiver: crate::Receiver<ServerUpdateIn>,
    pub server_update_sender: crate::Sender<ServerUpdateOut>,
    pub player_access: Arc<Minecraft19PlayerAccess>,
}

impl AxolotlWorld {
    pub fn load(
        group: impl AsRef<str>,
        game: Arc<AxolotlGame>,
        directory: PathBuf,
        player_access: Arc<Minecraft19PlayerAccess>,
        generator: ChunkSettings,
    ) -> Result<WorldLoad, Error> {
        let (server_update_sender, server_update_receiver) = crate::unbounded();
        let (to_sever_update_sender, to_sever_update_receiver) = crate::unbounded();

        let accessor = Minecraft19WorldAccessor::load(game.clone(), directory)?;
        let generator = AxolotlGenerator::new(game, generator);
        let name = accessor.world.level_dat.level_name.clone();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let name_hash = hasher.finish();
        let world = AxolotlWorld {
            full_name: format!("{}/{}", group.as_ref(), name),
            name,
            name_hash,
            clients: Default::default(),
            render_distance: 8,
            simulation_distance: 8,
            game_world: ECSWorld::new(),
            chunk_map: Arc::new(ChunkMap::new(generator, accessor)),
            chunk_tickets: Default::default(),
            server_update_receiver,
            server_update_sender: to_sever_update_sender,
            player_access,
        };
        Ok(WorldLoad {
            world,
            sender: server_update_sender,
            receiver: to_sever_update_receiver,
        })
    }

    pub fn create(
        game: Arc<AxolotlGame>,
        group: impl AsRef<str>,
        name: String,
        render_distance: u8,
        simulation_distance: u8,
        directory: PathBuf,
        chunk_generator: ChunkSettings,
        player_access: Arc<Minecraft19PlayerAccess>,
        seed: i64,
        dimension: OwnedNameSpaceKey,
    ) -> Result<WorldLoad, Error> {
        let mut dimensions = HashMap::new();
        dimensions.insert(
            dimension.clone(),
            Dimension {
                world_type: dimension,
                generator: serde_json::to_value(chunk_generator.clone())?,
                other: HashMap::new(),
            },
        );

        let settings = WorldGenSettings {
            seed,
            dimensions,
            generate_features: false,
            bonus_chest: false,
        };
        let generator = AxolotlGenerator::new(game.clone(), chunk_generator);
        let (server_update_sender, server_update_receiver) = crate::unbounded();
        let (to_sever_update_sender, to_sever_update_receiver) = crate::unbounded();
        let mut hasher = DefaultHasher::new();
        group.as_ref().hash(&mut hasher);
        name.hash(&mut hasher);
        let name_hash = hasher.finish();
        let world = Self {
            full_name: format!("{}/{}", group.as_ref(), name),
            name: name.clone(),
            name_hash,
            clients: Default::default(),
            render_distance,
            simulation_distance,
            game_world: ECSWorld::new(),
            chunk_map: Arc::new(ChunkMap::new(
                generator,
                Minecraft19WorldAccessor::create(game, settings, directory, name)?,
            )),
            chunk_tickets: Default::default(),
            player_access,
            server_update_receiver,
            server_update_sender: to_sever_update_sender,
        };
        Ok(WorldLoad {
            world,
            sender: server_update_sender,
            receiver: to_sever_update_receiver,
        })
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
                if let Err(error) = player.sender.send(update.clone()) {
                    warn!("Failed to send chunk update to player: {}", error);
                }
            }
        }
    }
    pub fn tick_entities(&mut self) {}
    fn handle_updates(&mut self) {
        let receiver = self.server_update_receiver.clone();
        for update in receiver.try_iter() {
            self.handle_update(update);
        }
    }
    fn handle_player_locations(&mut self) {
        for player in self.clients.iter() {
            let pos = self
                .game_world
                .entity(player.entity)
                .expect("Player entity not found");
            let mut pos = pos.get::<&mut Location>().unwrap();
            let active_location = player.player_location.lock();
            if active_location.eq(&pos) {
                continue;
            }
            pos.update_from_ref(&active_location);
            // TODO update nearby players
        }
    }
    fn handle_update(&mut self, update: ServerUpdateIn) {
        match update {
            ServerUpdateIn::NewPlayer { sender, uuid } => {
                let player = self.player_access.get_player(uuid, self.name_hash);
                let player = match player {
                    Ok(ok) => match ok {
                        None => {
                            warn!("Attempted to join world while in another world");
                            return;
                        }
                        Some(ok) => ok,
                    },
                    Err(err) => {
                        warn!("Failed to get player: {}", err);
                        sender.send(Arc::new(PlayerUpdate::FailedToLoadPlayer));
                        return;
                    }
                };
                let game_player = GamePlayer::from(player);

                let shared_location = Arc::new(Mutex::new(game_player.position.clone()));
                sender.send(Arc::new(PlayerUpdate::Location(shared_location.clone())));
                let entity = self.game_world.spawn(game_player);
                let player = WorldPlayer {
                    entity,
                    sender,
                    uuid,
                    player_location: shared_location,
                };
                self.clients.push(Arc::new(player));
            }
            ServerUpdateIn::PlayerLeft(uuid) => {
                let option = self.clients.iter().position(|client| client.uuid == uuid);
                if let Some(entity) = option {
                    let player = self.clients.remove(entity);
                    self.chunk_tickets.remove_player(&player);
                    // We should be able to unwrap here because we should have the only reference to the entity
                    let player = match Arc::try_unwrap(player) {
                        Ok(ok) => ok,
                        Err(err) => {
                            warn!("Player Reference is still alive");
                            // TODO: Handle this better
                            return;
                        }
                    };
                    let entity = self.game_world.remove::<GamePlayer>(player.entity).unwrap();
                    self.game_world.despawn(player.entity);
                    let data: PlayerData = entity.into();
                    self.player_access.save_player(uuid, &data);
                } else {
                    warn!("Player left but was not found in world");
                }
            }
            ServerUpdateIn::UnloadChunk { x, z } => {
                let mut entities_to_remove = Vec::new();
                self.game_world
                    .query::<(&MinecraftEntity, &Location)>()
                    .iter()
                    .for_each(|(entity, (_, location))| {
                        let chunk_x = location.x as i32 / 16;
                        let chunk_z = location.z as i32 / 16;
                        if chunk_x == x && chunk_z == z {
                            entities_to_remove.push(entity);
                        }
                    });
            }
        }
    }
}

impl Hash for AxolotlWorld {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_name.hash(state);
    }
}

impl Debug for AxolotlWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AxolotlWorld")
            .field("name", &self.name)
            .field("full_name", &self.full_name)
            .field("clients", &self.clients)
            .field("render_distance", &self.render_distance)
            .field("simulation_distance", &self.simulation_distance)
            .field("chunk_map", &self.chunk_map)
            .field("chunk_tickets", &self.chunk_tickets)
            .field("player_access", &self.player_access)
            .finish()
    }
}

impl World for AxolotlWorld {
    type Chunk = AxolotlChunk;
    type WorldBlock = PlacedBlock;
    type NoiseGenerator = AxolotlGenerator;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn tick(&mut self) {
        self.handle_player_locations();
        self.handle_updates();
        // TODO: Do Game Tick including: entities, liquids, etc
    }

    fn generator(&self) -> &Self::NoiseGenerator {
        &self.chunk_map.generator
    }

    fn set_block(
        &self,
        location: BlockPosition,
        block: PlacedBlock,
        required_loaded: bool,
    ) -> bool {
        let mut relative_pos = location;
        let position = (relative_pos).chunk();
        let id = block.id();

        if let Some(value) = self.chunk_map.thread_safe_chunks.get(&position) {
            let mut guard = value.val().value.write();
            guard.set_block(relative_pos, block);
            drop(guard);
            drop(value);
            self.send_block_update(location, id);
            true
        } else if !required_loaded {
            debug!("Chunk not loading. Will load chunk and set block");
            self.chunk_map.queue.push(ChunkUpdate::Load {
                x: position.x(),
                z: position.z(),
                set_block: Some((location, block)),
            });
            true
        } else {
            false
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
                block_len.push((pos, block.id()));
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
