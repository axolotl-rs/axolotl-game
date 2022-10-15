use crate::world::entity::properties::{Food, Health, Location};
use ahash::AHashSet;

use dumbledore::component::Component;

use dumbledore::Bundle;

#[derive(Debug, Clone)]
pub struct Chunks(pub AHashSet<(i64, i64)>);
impl Component for Chunks {}

#[derive(Debug, Clone, Bundle)]
pub struct GamePlayer {
    pub food: Food,
    pub health: Health,
    pub position: Location,
    pub tracked_chunks: Chunks,
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
    UpdateBlock {
        // TODO data
    },
    UpdateEntity {
        // TODO data
    },
}
