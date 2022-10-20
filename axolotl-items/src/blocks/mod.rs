pub mod generic_block;
pub mod hard_coded;
pub mod material;
pub(crate) mod raw_state;

use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::Item;
use axolotl_api::world::BlockPosition;
use axolotl_api::{NameSpaceRef, NamespacedKey, OwnedNameSpaceKey};

use crate::blocks::generic_block::VanillaState;
use crate::blocks::hard_coded::HardCodedBlock;
use ahash::AHashMap;
use generic_block::GenericBlock;

#[derive(Debug)]
pub enum MinecraftBlock {
    Air,
    GenericBlock(GenericBlock),
    HardCodedBlock(Box<dyn HardCodedBlock>),
}
impl PartialEq for MinecraftBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Item for MinecraftBlock {
    fn id(&self) -> usize {
        match self {
            MinecraftBlock::Air => 0,
            MinecraftBlock::GenericBlock(block) => block.0.id,
            MinecraftBlock::HardCodedBlock(d) => d.id(),
        }
    }
    fn get_namespace(&self) -> NameSpaceRef<'_> {
        match self {
            MinecraftBlock::Air => NameSpaceRef::new("minecraft", "air"),
            MinecraftBlock::GenericBlock(v) => v.get_namespace(), //MinecraftBlock::DynBlock(block) => block.get_namespace(),
            MinecraftBlock::HardCodedBlock(v) => v.get_namespace(),
        }
    }
}

impl Block for MinecraftBlock {
    type State = VanillaState;

    fn get_default_state(&self) -> Self::State {
        match self {
            MinecraftBlock::Air => {
                let mut state = VanillaState::default();
                state.state_id = 0;
                state
            }
            MinecraftBlock::GenericBlock(v) => v.get_default_state(),
            MinecraftBlock::HardCodedBlock(v) => v.get_default_state(),
        }
    }
}
