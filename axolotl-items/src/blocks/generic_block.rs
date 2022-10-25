use crate::blocks::raw_state::RawState;
use crate::materials::BlockMaterial;
use ahash::{AHashMap, HashMap};
use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::{Item, ItemType};
use axolotl_api::{NameSpaceRef, NamespacedId, NumericId};
use axolotl_data_rs::blocks::{Block as RawBlock, Material};
use serde::de::{MapAccess, Visitor};
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt::Formatter;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DropType {
    #[default]
    SelfDrop,
    DropOne(usize),
    DropMany(Vec<usize>),
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VanillaState {
    pub state_id: usize,
    pub values: AHashMap<String, BlockStateValue>,
    pub default: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub enum VanillaStateIdOrValue {
    Id(usize),
    Value(VanillaState),
}

pub struct VanillaStateVisitor;

impl<'de> Visitor<'de> for VanillaStateVisitor {
    type Value = VanillaState;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("VanillaState")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = AHashMap::new();
        let mut id = 0;
        let mut default = false;
        while let Some(key) = map.next_key::<String>()? {
            if key.eq("id") {
                id = map.next_value::<usize>()?;
            } else if key.eq("properties") {
                values = map.next_value()?;
            } else if key.eq("default") {
                default = map.next_value::<bool>()?;
            } else {
                map.next_value::<serde_json::Value>()?;
            }
        }
        Ok(VanillaState {
            state_id: id,
            values,
            default,
        })
    }
}

impl<'de> Deserialize<'de> for VanillaState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(VanillaStateVisitor)
    }
}

impl VanillaState {}
impl BlockState for VanillaState {
    fn get(&self, name: &str) -> Option<&BlockStateValue> {
        self.values.get(name)
    }

    fn set(&mut self, name: impl Into<String>, value: BlockStateValue) {
        self.values.insert(name.into(), value);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundingBox {
    Block,
}
#[derive(Debug, Clone)]
pub struct BlockProperties {
    pub material: Arc<Material>,

    pub key: String,
    pub id: usize,
    pub default_state: usize,
    pub states: Vec<VanillaState>,
}

#[derive(Debug, Clone)]
pub struct GenericBlock(pub(crate) BlockProperties);
impl NamespacedId for GenericBlock {
    fn namespace(&self) -> &str {
        "minecraft"
    }

    fn key(&self) -> &str {
        &self.0.key
    }
}
impl NumericId for GenericBlock {
    fn id(&self) -> usize {
        self.0.id
    }
}
impl GenericBlock {
    pub fn new(
        raw_block: RawBlock,
        materials: &HashMap<String, Arc<Material>>,
        raw_states: &mut std::collections::HashMap<String, RawState>,
    ) -> Self {
        let mut states = Vec::new();
        let mut default_state = 0;
        let raw_state = raw_states.remove(&format!("minecraft:{}", raw_block.name));
        if let Some(raw_state) = raw_state {
            for (index, state) in raw_state.states.into_iter().enumerate() {
                if state.default {
                    default_state = index;
                }
                states.push(state);
            }
        }

        let value = BlockProperties {
            material: materials
                .get(&raw_block.properties.material)
                .expect("Material not found")
                .clone(),
            id: raw_block.id as usize,
            key: raw_block.name,
            states,
            default_state,
        };
        Self(value)
    }
}
impl ItemType for GenericBlock {}

impl Block for GenericBlock {
    type State = VanillaState;

    fn create_default_state(&self) -> Self::State {
        self.0.states[self.0.default_state].clone()
    }

    fn get_default_state(&self) -> Cow<'_, Self::State> {
        Cow::Borrowed(&self.0.states[self.0.default_state])
    }
}
