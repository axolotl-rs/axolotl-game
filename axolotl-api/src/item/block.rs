use serde_json::ser::State;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::item::Item;
use crate::world::{BlockPosition, GenericLocation, World, WorldLocation};
use crate::world_gen::noise::ChunkGenerator;
use crate::NameSpaceRef;

/// A Generic Block State Type
#[derive(Debug, Clone, PartialEq)]
pub enum BlockStateValue {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

pub trait BlockState: Debug {
    fn get(&self, name: &str) -> Option<&BlockStateValue>;

    fn set(&mut self, name: impl Into<String>, value: BlockStateValue);
}

pub trait Block: Item {
    type State: BlockState;
    type PlacedBlock: Clone;

    fn get_default_placed_block(&self) -> Self::PlacedBlock;

    fn get_default_state(&self) -> Self::State;
}

impl<'s, B> Block for &'s B
where
    B: Block,
{
    type State = B::State;
    type PlacedBlock = B::PlacedBlock;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        (*self).get_default_placed_block()
    }

    fn get_default_state(&self) -> Self::State {
        (*self).get_default_state()
    }
}
impl<B: Block> Block for Box<B> {
    type State = B::State;

    type PlacedBlock = B::PlacedBlock;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        (**self).get_default_placed_block()
    }

    fn get_default_state(&self) -> Self::State {
        (**self).get_default_state()
    }
}
