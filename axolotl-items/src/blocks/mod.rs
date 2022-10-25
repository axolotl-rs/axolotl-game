pub mod generic_block;
pub mod hard_coded;
pub(crate) mod raw_state;

use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::{Item, ItemType};
use axolotl_api::world::BlockPosition;
use axolotl_api::{NameSpaceRef, NamespacedId, NamespacedKey, NumericId, OwnedNameSpaceKey};
use std::borrow::Cow;
use std::sync::Arc;

use crate::blocks::generic_block::VanillaState;
use crate::blocks::hard_coded::HardCodedBlock;
use ahash::AHashMap;
use generic_block::GenericBlock;

pub type MinecraftBlock = Arc<InnerMinecraftBlock>;

#[derive(Debug)]
pub enum InnerMinecraftBlock {
    Air,
    GenericBlock(GenericBlock),
    HardCodedBlock(Box<dyn HardCodedBlock>),
}
impl NamespacedId for InnerMinecraftBlock {
    fn namespace(&self) -> &str {
        match self {
            InnerMinecraftBlock::Air => "minecraft",
            InnerMinecraftBlock::GenericBlock(block) => block.namespace(),
            InnerMinecraftBlock::HardCodedBlock(block) => block.namespace(),
        }
    }

    fn key(&self) -> &str {
        match self {
            InnerMinecraftBlock::Air => "air",
            InnerMinecraftBlock::GenericBlock(block) => block.key(),
            InnerMinecraftBlock::HardCodedBlock(block) => block.key(),
        }
    }
}
impl PartialEq for InnerMinecraftBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl<'game> ItemType for InnerMinecraftBlock {}

impl NumericId for InnerMinecraftBlock {
    fn id(&self) -> usize {
        match self {
            InnerMinecraftBlock::Air => 0,
            InnerMinecraftBlock::GenericBlock(block) => block.id(),
            InnerMinecraftBlock::HardCodedBlock(block) => block.id(),
        }
    }
}

impl Block for InnerMinecraftBlock {
    type State = VanillaState;

    fn create_default_state(&self) -> Self::State {
        match self {
            InnerMinecraftBlock::Air => {
                let mut state = VanillaState::default();
                state.state_id = 0;
                state
            }
            InnerMinecraftBlock::GenericBlock(v) => v.create_default_state(),
            InnerMinecraftBlock::HardCodedBlock(v) => v.create_default_state(),
        }
    }
    fn get_default_state(&self) -> Cow<'_, Self::State> {
        match self {
            InnerMinecraftBlock::Air => Cow::Owned(VanillaState::default()),
            InnerMinecraftBlock::GenericBlock(v) => v.get_default_state(),
            InnerMinecraftBlock::HardCodedBlock(v) => v.get_default_state(),
        }
    }
}
