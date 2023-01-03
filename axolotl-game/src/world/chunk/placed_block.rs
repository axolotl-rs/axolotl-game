use axolotl_api::world::World;
use axolotl_api::{NamespacedId, NumericId, OwnedNameSpaceKey};
use axolotl_items::blocks::generic_block::VanillaStateIdOrValue;
use axolotl_items::blocks::{InnerMinecraftBlock, MinecraftBlock};
use axolotl_world::chunk::PaletteItem;

use crate::AxolotlGame;

#[derive(Debug, PartialEq)]
pub struct PlacedBlock<W: World> {
    pub state: VanillaStateIdOrValue,
    pub block: MinecraftBlock<AxolotlGame<W>>,
}
impl<W: World> Clone for PlacedBlock<W> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            block: self.block.clone(),
        }
    }
}
impl<W: World> From<PlacedBlock<W>> for PaletteItem {
    fn from(val: PlacedBlock<W>) -> Self {
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

impl<W: World> From<MinecraftBlock<AxolotlGame<W>>> for PlacedBlock<W> {
    fn from(block: MinecraftBlock<AxolotlGame<W>>) -> Self {
        PlacedBlock {
            state: VanillaStateIdOrValue::Id(
                <InnerMinecraftBlock<AxolotlGame<W>> as axolotl_api::item::block::Block<
                    AxolotlGame<W>,
                >>::get_default_state(&block)
                .as_ref()
                .state_id,
            ),
            block,
        }
    }
}

impl<W: World> PlacedBlock<W> {
    pub fn is_air(&self) -> bool {
        <InnerMinecraftBlock<AxolotlGame<W>> as axolotl_api::item::block::Block<AxolotlGame<W>>>::is_air(
            &self.block,
        )
    }
    pub fn id(&self) -> usize {
        self.block.id()
    }
}
