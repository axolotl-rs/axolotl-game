use std::sync::Arc;

use ahash::HashMap;
use axolotl_data_rs::blocks::{Block as RawBlock, Material};

use axolotl_api::events::{Event, EventHandler};
use axolotl_api::game::Game;
use axolotl_api::item::block::{Block, BlockPlaceEvent};
use axolotl_api::item::ItemType;
use axolotl_api::{NamespacedId, NumericId};

use crate::blocks::generic_block::{BlockProperties, VanillaState};
use crate::blocks::raw_state::RawState;

#[derive(Debug, Clone)]
pub struct BedBlock {
    pub id: usize,
    pub default_state: usize,
    pub states: Vec<VanillaState>,
    pub material: Arc<Material>,
    pub key: String,
}
impl BedBlock {
    pub fn new(
        raw_block: RawBlock,
        materials: &HashMap<String, Arc<Material>>,
        raw_states: &mut std::collections::HashMap<String, RawState>,
    ) -> Self {
        let (states, default_state) = BlockProperties::process_state(&raw_block.name, raw_states);
        BedBlock {
            id: raw_block.id,
            default_state,
            states,
            key: raw_block.name,
            material: materials
                .get(&raw_block.properties.material)
                .expect("Material not found")
                .clone(),
        }
    }
}

impl ItemType for BedBlock {}

impl NumericId for BedBlock {
    fn id(&self) -> usize {
        self.id
    }
}

impl<G: Game> EventHandler<BlockPlaceEvent<'_, G>> for BedBlock {
    fn handle(
        &self,
        _event: BlockPlaceEvent<G>,
    ) -> Result<<BlockPlaceEvent<G> as Event>::Result, <BlockPlaceEvent<G> as Event>::Error> {
        Ok(false)
    }
}

impl NamespacedId for BedBlock {
    fn namespace(&self) -> &str {
        "minecraft"
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl<G: Game> Block<G> for BedBlock {
    type State = VanillaState;

    fn create_default_state(&self) -> Self::State {
        self.states[self.default_state].clone()
    }

    fn is_air(&self) -> bool {
        false
    }
}
