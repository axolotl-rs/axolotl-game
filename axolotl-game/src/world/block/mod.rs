pub mod material;

use crate::world::chunk::placed_block::PlacedBlock;
use crate::AxolotlWorld;
use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::Item;
use axolotl_api::world::BlockPosition;
use axolotl_api::{NameSpaceRef, NamespacedKey};

use ahash::AHashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MapState {
    pub map: AHashMap<String, BlockStateValue>,
}
impl Default for MapState {
    fn default() -> Self {
        Self {
            map: AHashMap::new(),
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
#[derive(Debug, PartialEq)]
pub enum MinecraftBlock {
    Air,
    GenericBlock(NameSpaceRef<'static>, GenericBlock),
    //DynBlock(Box<dyn AxolotlBlock<State = MapState, PlacedBlock = PlacedBlock>>),
}
impl MinecraftBlock {
    pub fn from_id(_id: impl Into<u64>) -> Option<&'static MinecraftBlock> {
        todo!("Define via magic(a macro)")
    }
    pub fn from_namespace(_id: &impl NamespacedKey) -> Option<&'static MinecraftBlock> {
        todo!("Define via magic(a macro)")
    }
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
            //MinecraftBlock::DynBlock(v) => v.get_namespace(),
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
        }
    }

    fn get_default_state(&self) -> Self::State {
        match self {
            _ => MapState::default(),
        }
    }
}
impl AxolotlBlock for &'static MinecraftBlock {
    fn on_block_placed(
        &self,
        _block_state: &Self::State,
        _location: &BlockPosition,
        _world: &AxolotlWorld,
    ) {
        match self {
            _ => {}
        }
    }
}

impl<Ab: AxolotlBlock> AxolotlBlock for Box<Ab> {}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockProperties {}
#[derive(Debug, Clone, PartialEq)]
pub struct GenericBlock(BlockProperties);
