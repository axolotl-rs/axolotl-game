use crate::world::block::{MapState, MinecraftBlock};
use crate::world::chunk::section::InvalidChunkSection;
use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::NameSpaceRef;
use axolotl_world::chunk::PaletteItem;

#[derive(Debug, Clone, PartialEq)]
pub struct PlacedBlock {
    pub state: MapState,
    pub block: &'static MinecraftBlock,
}
impl Default for PlacedBlock {
    fn default() -> Self {
        Self {
            state: MapState::default(),
            block: MinecraftBlock::from_id(0u64).unwrap(),
        }
    }
}
impl From<&'static MinecraftBlock> for PlacedBlock {
    fn from(block: &'static MinecraftBlock) -> Self {
        PlacedBlock {
            state: block.get_default_state(),
            block,
        }
    }
}
impl TryFrom<&'_ PaletteItem> for PlacedBlock {
    type Error = InvalidChunkSection;

    fn try_from(value: &'_ PaletteItem) -> Result<Self, Self::Error> {
        let option = MinecraftBlock::from_namespace(&value.name);
        if let Some(block) = option {
            // TODO load state
            Ok(PlacedBlock {
                state: block.get_default_state(),
                block,
            })
        } else {
            // TODO: Add a better error
            Err(InvalidChunkSection::InvalidBlock(0))
        }
    }
}
impl PlacedBlock {
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

impl Item for PlacedBlock {
    fn get_namespace(&self) -> NameSpaceRef<'static> {
        self.block.get_namespace()
    }
}

impl Block for PlacedBlock {
    type State = MapState;
    type PlacedBlock = Self;

    fn get_default_placed_block(&self) -> Self::PlacedBlock {
        self.clone()
    }

    fn get_default_state(&self) -> Self::State {
        self.state.clone()
    }
}
