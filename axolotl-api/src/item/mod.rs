use crate::{NameSpaceRef, NamespacedKey, NumericId};
use std::borrow::Cow;
use std::fmt::Debug;

pub mod block;
pub mod recipes;
pub mod vanilla;

pub trait ItemType: Debug + Send + Sync {}
impl<IT: ItemType> ItemType for &'_ IT {}
pub trait ToolType {
    fn name() -> &'static str;
}

pub trait Item: ItemType + NumericId {}

pub trait HasHarvestLevel {
    fn get_harvest_level() -> f32;
}

pub trait Tool: Item + HasHarvestLevel {
    type ToolType: ToolType;
}

impl<'s, B> Item for &'s B where B: Item + NumericId {}
