use crate::blocks::generic_block::VanillaState;
use crate::blocks::MinecraftBlock;
use axolotl_api::item::block::Block;
use axolotl_api::NameSpaceRef;

pub mod bed_block;

pub trait HardCodedBlock: Block<State = VanillaState> {}

pub fn register_hard_coded(id: u32) -> Option<MinecraftBlock> {
    match id {
        90 => Some(MinecraftBlock::HardCodedBlock(Box::new(
            bed_block::WhiteBed,
        ))),
        _ => None,
    }
}
