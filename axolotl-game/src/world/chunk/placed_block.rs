use crate::AxolotlGame;

use axolotl_api::{NamespacedId, NumericId, OwnedNameSpaceKey};
use axolotl_items::blocks::generic_block::VanillaStateIdOrValue;
use axolotl_items::blocks::{InnerMinecraftBlock, MinecraftBlock};
use axolotl_world::chunk::PaletteItem;

#[derive(Debug, Clone, PartialEq)]
pub struct PlacedBlock {
    pub state: VanillaStateIdOrValue,
    pub block: MinecraftBlock<AxolotlGame>,
}
impl From<PlacedBlock> for PaletteItem {
    fn from(val: PlacedBlock) -> Self {
        // TODO convert to palette item properly
        PaletteItem {
            name: OwnedNameSpaceKey::new(
                val.block.namespace().to_string(),
                val.block.key().to_string(),
            ),
            properties: Default::default(),
        }
    }
}

impl From<MinecraftBlock<AxolotlGame>> for PlacedBlock {
    fn from(block: MinecraftBlock<AxolotlGame>) -> Self {
        PlacedBlock {
            state: VanillaStateIdOrValue::Id(
                <InnerMinecraftBlock<AxolotlGame> as axolotl_api::item::block::Block<
                    AxolotlGame,
                >>::get_default_state(&block)
                .as_ref()
                .state_id,
            ),
            block,
        }
    }
}

impl PlacedBlock {
    pub fn is_air(&self) -> bool {
        <InnerMinecraftBlock<AxolotlGame> as axolotl_api::item::block::Block<AxolotlGame>>::is_air(
            &self.block,
        )
    }
    pub fn id(&self) -> usize {
        self.block.id()
    }
}
