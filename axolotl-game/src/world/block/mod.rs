pub mod material;

use crate::world::chunk::PlacedBlock;
use crate::AxolotlWorld;
use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::Item;
use axolotl_api::world::BlockPosition;
use axolotl_api::NameSpaceRef;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MapState {
    pub map: HashMap<String, BlockStateValue>,
}
impl Default for MapState {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
impl BlockState for MapState {
    fn get(&self, name: &str) -> Option<&BlockStateValue> {
        self.map.get(name)
    }

    fn set(&mut self, name: impl Into<String>, value: BlockStateValue) {
        self.map.insert(name.into(), value);
    }
}

pub trait AxolotlBlock: Block {
    fn on_block_placed(
        &self,
        _block_state: &Self::State,
        _location: &BlockPosition,
        _world: &AxolotlWorld,
    ) {
        // By default, do nothing
    }
}
#[derive(Debug)]
pub enum MinecraftBlock {
    Air,
    GenericBlock(NameSpaceRef<'static>, GenericBlock),
    DynBlock(Box<dyn AxolotlBlock<State = MapState, PlacedBlock = PlacedBlock>>),
}
impl MinecraftBlock {
    pub fn id(&self) -> usize {
        match self {
            MinecraftBlock::Air => 0,
            _ => todo!(),
        }
    }
}

impl Item for MinecraftBlock {
    fn get_namespace(&self) -> NameSpaceRef<'static> {
        match self {
            MinecraftBlock::Air => NameSpaceRef::new("minecraft", "air"),
            MinecraftBlock::GenericBlock(key, _) => key.clone(),
            MinecraftBlock::DynBlock(v) => v.get_namespace(),
        }
    }
}
impl Block for &'static MinecraftBlock {
    type State = MapState;

    type PlacedBlock = PlacedBlock;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        match self {
            MinecraftBlock::Air => PlacedBlock {
                state: MapState::default(),
                block: self,
            },
            MinecraftBlock::GenericBlock(_, _block) => PlacedBlock {
                state: MapState::default(),
                block: self,
            },
            MinecraftBlock::DynBlock(v) => PlacedBlock {
                state: v.get_default_state(),
                block: self,
            },
        }
    }

    fn get_default_state(&self) -> Self::State {
        match self {
            MinecraftBlock::DynBlock(v) => v.get_default_state(),
            _ => MapState::default(),
        }
    }
}
impl AxolotlBlock for &'static MinecraftBlock {
    fn on_block_placed(
        &self,
        block_state: &Self::State,
        location: &BlockPosition,
        world: &AxolotlWorld,
    ) {
        match self {
            MinecraftBlock::DynBlock(v) => v.on_block_placed(block_state, location, world),
            _ => {}
        }
    }
}

impl<Ab: AxolotlBlock> AxolotlBlock for Box<Ab> {}

#[derive(Debug, Clone)]
pub struct BlockProperties {}
#[derive(Debug, Clone)]
pub struct GenericBlock(BlockProperties);
