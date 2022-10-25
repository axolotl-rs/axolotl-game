use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::{NamespacedId, NamespacedKey, NumericId, OwnedNameSpaceKey};
use axolotl_items::blocks::generic_block::{VanillaState, VanillaStateIdOrValue};
use axolotl_items::blocks::{InnerMinecraftBlock, MinecraftBlock};
use axolotl_world::chunk::PaletteItem;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedBlock {
    pub state: VanillaStateIdOrValue,
    pub block: MinecraftBlock,
}
impl Into<PaletteItem> for PlacedBlock {
    fn into(self) -> PaletteItem {
        // TODO convert to palette item properly
        PaletteItem {
            name: OwnedNameSpaceKey::new(
                self.block.namespace().to_string(),
                self.block.key().to_string(),
            ),
            properties: Default::default(),
        }
    }
}

impl From<MinecraftBlock> for PlacedBlock {
    fn from(block: MinecraftBlock) -> Self {
        PlacedBlock {
            state: VanillaStateIdOrValue::Id(block.get_default_state().as_ref().state_id),
            block,
        }
    }
}

impl PlacedBlock {
    pub fn is_air(&self) -> bool {
        match self.block.as_ref() {
            InnerMinecraftBlock::Air => true,
            _ => false,
        }
    }
    pub fn id(&self) -> usize {
        self.block.id()
    }
}
