use std::collections::HashMap;

use axolotl_api::item::block::BlockState;
use axolotl_api::world::BlockPosition;
use axolotl_api::OwnedNameSpaceKey;

#[derive(Debug)]
pub struct PlacedBlock {
    pub state: Box<dyn BlockState>,
    pub block: OwnedNameSpaceKey,
}

#[derive(Debug)]
pub struct AxolotlChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub blocks: HashMap<BlockPosition, PlacedBlock>,
}
