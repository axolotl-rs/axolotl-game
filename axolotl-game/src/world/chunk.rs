use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::NameSpaceRef;
use std::collections::HashMap;

use crate::world::block::MapState;
use crate::MinecraftBlock;
use axolotl_api::world::BlockPosition;

#[derive(Debug, Clone)]
pub struct PlacedBlock {
    pub state: MapState,
    pub block: &'static MinecraftBlock,
}

impl Item for PlacedBlock {
    fn get_namespace(&self) -> NameSpaceRef<'static> {
        self.block.get_namespace()
    }
}

impl Block for PlacedBlock {
    type State = MapState;
    type PlacedBlock = Self;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        self.clone()
    }

    fn get_default_state(&self) -> Self::State {
        self.state.clone()
    }
}
#[derive(Debug)]
pub struct AxolotlChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub blocks: HashMap<BlockPosition, PlacedBlock>,
}
