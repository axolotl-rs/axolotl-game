use crate::blocks::generic_block::VanillaState;
use crate::blocks::MinecraftBlock;
use axolotl_api::item::block::Block;
use axolotl_api::{NameSpaceRef, NamespacedId, NumericId};

pub mod bed_block;

pub trait HardCodedBlock: Block<State = VanillaState> + NumericId + NamespacedId {}

pub fn register_hard_coded(id: usize) -> Option<MinecraftBlock> {
    match id {
        _ => None,
    }
}
