use crate::world::entity::properties::{Food, Health, Location};
use ahash::{AHashMap, AHashSet};

use dumbledore::component::Component;

use axolotl_api::world::BlockPosition;
use axolotl_world::entity::player::PlayerData;
use dumbledore::Bundle;

#[derive(Debug, Clone)]
pub struct Chunks(pub AHashSet<(i64, i64)>);

impl Component for Chunks {}

#[derive(Debug, Clone, Bundle)]
pub struct GamePlayer {
    pub food: Food,
    pub health: Health,
    pub position: Location,
}
impl From<PlayerData> for GamePlayer {
    fn from(_data: PlayerData) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum PlayerUpdate {
    LoadChunk {
        // TODO data
    },
    UnloadChunk {
        // TODO data
    },
    UpdateChunk {
        // TODO data
    },
    UpdateBlock(BlockPosition, usize),
    UpdateEntity {
        // TODO data
    },
    SectionUpdate(AHashMap<i64, Vec<i64>>),
}
