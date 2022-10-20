use crate::blocks::raw_state::RawState;
use ahash::{AHashMap, HashMap};
use axolotl_api::item::block::{Block, BlockState, BlockStateValue};
use axolotl_api::item::Item;
use axolotl_api::NameSpaceRef;
use minecraft_data_rs::models::block::BoundingBox;
use serde::de::{MapAccess, Visitor};
use serde::Deserialize;
use std::fmt::{format, Formatter};

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
        while let Some(key) = map.next_key::<String>()? {
            if key.eq("id") {
                id = map.next_value::<usize>()?;
            } else if key.eq("properties") {
                values = map.next_value()?;
            } else {
                map.next_value::<serde_json::Value>()?;
            }
        }
        Ok(VanillaState {
            state_id: id,
            values,
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

#[derive(Debug, Clone)]
pub struct BlockProperties {
    pub resistance: f32,
    pub hardness: f32,
    pub stack_size: u32,
    pub diggable: bool,
    // Will be None if value is "default"
    pub material: Option<String>,
    pub transparent: bool,
    pub emit_light: u32,
    pub filter_light: u32,
    pub default_state: usize,
    pub states: Vec<VanillaState>,
    pub drops: DropType,
    // Will be None if "block"
    pub bounding_box: BoundingBox,

    pub display_name: String,
    pub id_name: String,
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct GenericBlock(pub(crate) BlockProperties);

impl GenericBlock {
    pub fn new(
        raw_block: minecraft_data_rs::models::block::Block,
        raw_states: &mut std::collections::HashMap<String, RawState>,
    ) -> Self {
        let mut states = Vec::new();
        let mut default_state = 0;
        let raw_state = raw_states.remove(&format!("minecraft:{}", raw_block.name));
        if let Some(raw_state) = raw_state {
            for (index, state) in raw_state.states.into_iter().enumerate() {
                if raw_block.default_state.unwrap_or_default() as usize == state.state_id {
                    default_state = index;
                }
                states.push(state);
            }
        }

        let mut drops = if raw_block.drops.len() == 1 {
            let drop = raw_block.drops[0];
            if drop == raw_block.id {
                DropType::SelfDrop
            } else {
                DropType::DropOne(drop as usize)
            }
        } else {
            DropType::DropMany(raw_block.drops.into_iter().map(|v| v as usize).collect())
        };

        let value = BlockProperties {
            resistance: raw_block.blast_resistance.unwrap_or_default(),
            hardness: raw_block.hardness.unwrap_or_default(),
            stack_size: raw_block.stack_size as u32,
            diggable: raw_block.diggable,
            material: raw_block.material,
            transparent: raw_block.transparent,
            emit_light: raw_block.emit_light as u32,
            filter_light: raw_block.filter_light as u32,
            default_state,
            states,
            drops,
            bounding_box: raw_block.bounding_box,
            display_name: raw_block.display_name,
            id_name: raw_block.name,
            id: raw_block.id as usize,
        };
        Self(value)
    }
}

impl Item for GenericBlock {
    fn id(&self) -> usize {
        self.0.id
    }

    fn get_namespace(&self) -> NameSpaceRef<'_> {
        NameSpaceRef::new("minecraft", &self.0.id_name)
    }
}

impl Block for GenericBlock {
    type State = VanillaState;

    fn get_default_state(&self) -> Self::State {
        self.0.states[self.0.default_state].clone()
    }
}
