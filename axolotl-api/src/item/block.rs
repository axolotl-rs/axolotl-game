use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use auto_impl::auto_impl;
pub use minecraft_protocol::packets::play::client::chunk::GetVanillaId;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};

use crate::events::{Event, EventHandler, NoError};
use crate::game::Game;
use crate::item::ItemType;
use crate::world::BlockPosition;
use crate::{NamespacedId, NumericId};

/// A Generic Block State Type
#[derive(Debug, Clone, PartialEq)]
pub enum BlockStateValue {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
pub struct BlockStateVisitor;
impl<'de> Visitor<'de> for BlockStateVisitor {
    type Value = BlockStateValue;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("BlockStateValue")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Float(v as f32))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::String(v))
    }
}
impl<'de> Deserialize<'de> for BlockStateValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BlockStateVisitor)
    }
}

pub trait BlockState: Debug + Clone {
    fn get(&self, name: &str) -> Option<&BlockStateValue>;

    fn set(&mut self, name: impl Into<String>, value: BlockStateValue);
}
/// Block Place Event
pub struct BlockPlaceEvent<'game, G: Game> {
    pub location: BlockPosition,
    pub world: &'game G::World,
    pub block: G::Block,
    pub item_stack: &'game mut G::ItemStack,
}
impl<G: Game> Event for BlockPlaceEvent<'_, G> {
    type Error = NoError;
    type Result = bool;

    fn get_name() -> &'static str {
        "block_place"
    }
}

#[auto_impl(&, Arc)]
pub trait Block<G: Game>:
    ItemType + NamespacedId + NumericId + for<'event> EventHandler<BlockPlaceEvent<'event, G>>
{
    type State: BlockState;

    fn create_default_state(&self) -> Self::State;

    fn is_air(&self) -> bool;

    fn get_default_state(&self) -> Cow<'_, Self::State> {
        Cow::Owned(self.create_default_state())
    }
}
