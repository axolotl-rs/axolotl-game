use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::{NamespacedKey, OwnedNameSpaceKey};
use axolotl_items::blocks::generic_block::VanillaState;
use axolotl_items::blocks::MinecraftBlock;
use axolotl_world::chunk::PaletteItem;

#[derive(Debug, Clone, PartialEq)]
pub struct PlacedBlock<'game> {
    pub state: VanillaState,
    pub block: &'game MinecraftBlock,
}
impl Into<PaletteItem> for PlacedBlock<'_> {
    fn into(self) -> PaletteItem {
        // TODO convert to palette item properly
        let space_ref = self.block.get_namespace();
        PaletteItem {
            name: OwnedNameSpaceKey::new(
                space_ref.get_namespace().to_string(),
                space_ref.get_key().to_string(),
            ),
            properties: Default::default(),
        }
    }
}
impl Default for PlacedBlock<'_> {
    fn default() -> Self {
        Self {
            state: VanillaState::default(),
            block: &MinecraftBlock::Air,
        }
    }
}
impl<'game> From<&'game MinecraftBlock> for PlacedBlock<'game> {
    fn from(block: &'game MinecraftBlock) -> Self {
        PlacedBlock {
            state: block.get_default_state(),
            block,
        }
    }
}

impl PlacedBlock<'_> {
    pub fn is_air(&self) -> bool {
        match self.block {
            MinecraftBlock::Air => true,
            _ => false,
        }
    }
    pub fn id(&self) -> usize {
        self.block.id()
    }
}
