use crate::world::entity::properties::{Food, Health, Location};
use ahash::{AHashMap, AHashSet};
use hecs::Bundle;

use axolotl_api::world::BlockPosition;
use axolotl_world::entity::player::PlayerData;

#[derive(Debug, Clone)]
pub struct Chunks(pub AHashSet<(i64, i64)>);

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

impl Into<PlayerData> for GamePlayer {
    fn into(self) -> PlayerData {
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
    FailedToLoadPlayer,
}
