pub mod generic_block;
pub(crate) mod raw_state;
pub mod v19;

use axolotl_api::item::block::{Block, BlockPlaceEvent};
use axolotl_api::item::ItemType;

use axolotl_api::{NamespacedId, NumericId};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

use crate::blocks::generic_block::VanillaState;

use axolotl_api::events::{EventHandler, NoError};
use axolotl_api::game::Game;
use generic_block::GenericBlock;

pub type MinecraftBlock<G> = Arc<InnerMinecraftBlock<G>>;

#[derive(Debug)]
pub enum InnerMinecraftBlock<G: Game> {
    Air { id: usize, key: String },
    GenericBlock(GenericBlock),
    DynamicBlock(Box<dyn Block<G, State = VanillaState>>),
}
impl<G: Game> NamespacedId for InnerMinecraftBlock<G> {
    fn namespace(&self) -> &str {
        match self {
            InnerMinecraftBlock::GenericBlock(block) => block.namespace(),
            InnerMinecraftBlock::DynamicBlock(v) => {
                let block = v.as_ref();
                block.namespace()
            }
            InnerMinecraftBlock::Air { .. } => "minecraft",
        }
    }

    fn key(&self) -> &str {
        match self {
            InnerMinecraftBlock::GenericBlock(block) => block.key(),
            InnerMinecraftBlock::DynamicBlock(v) => {
                let block = v.as_ref();
                block.key()
            }
            InnerMinecraftBlock::Air { key, .. } => key,
        }
    }
}
impl<G: Game> PartialEq for InnerMinecraftBlock<G> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl<'game, G: Game> ItemType for InnerMinecraftBlock<G> {}

impl<G: Game> NumericId for InnerMinecraftBlock<G> {
    fn id(&self) -> usize {
        match self {
            InnerMinecraftBlock::GenericBlock(block) => block.id(),
            InnerMinecraftBlock::DynamicBlock(id) => {
                let block = id.as_ref();
                block.id()
            }
            InnerMinecraftBlock::Air { id, .. } => *id,
        }
    }
}

impl<G: Game> EventHandler<BlockPlaceEvent<'_, G>> for InnerMinecraftBlock<G> {
    fn handle(&self, event: BlockPlaceEvent<G>) -> Result<bool, NoError> {
        match self {
            InnerMinecraftBlock::GenericBlock(block) => block.handle(event),
            InnerMinecraftBlock::DynamicBlock(b) => b.as_ref().handle(event),
            _ => {
                Ok(false) //??? Can't place air but it shouldnt be called
            }
        }
    }
}

impl<G: Game> Block<G> for InnerMinecraftBlock<G> {
    type State = VanillaState;

    fn create_default_state(&self) -> Self::State {
        match self {
            InnerMinecraftBlock::GenericBlock(v) => {
                <GenericBlock as Block<G>>::create_default_state(v)
            }
            InnerMinecraftBlock::DynamicBlock(v) => {
                <dyn Block<G, State = VanillaState> as Block<G>>::create_default_state(v.as_ref())
            }
            InnerMinecraftBlock::Air { .. } => VanillaState::default(),
        }
    }

    fn is_air(&self) -> bool {
        match self {
            InnerMinecraftBlock::GenericBlock(v) => <GenericBlock as Block<G>>::is_air(v),
            InnerMinecraftBlock::DynamicBlock(v) => {
                <dyn Block<G, State = VanillaState> as Block<G>>::is_air(v.as_ref())
            }
            InnerMinecraftBlock::Air { .. } => true,
        }
    }

    fn get_default_state(&self) -> Cow<'_, Self::State> {
        match self {
            InnerMinecraftBlock::GenericBlock(v) => {
                <GenericBlock as Block<G>>::get_default_state(v)
            }
            InnerMinecraftBlock::DynamicBlock(v) => {
                <dyn Block<G, State = VanillaState> as Block<G>>::get_default_state(v.as_ref())
            }
            InnerMinecraftBlock::Air { .. } => Cow::Owned(VanillaState::default()),
        }
    }
}
