use crate::{NameSpaceRef, NamespacedKey};
use std::fmt::Debug;

pub mod block;
pub mod food;
pub mod recipes;
pub mod vanilla;

pub trait ToolType {
    fn name() -> &'static str;
}

pub trait Item: Debug + Send + Sync {
    fn id(&self) -> usize;

    fn get_namespace(&self) -> NameSpaceRef<'_>;
}

pub trait HasHarvestLevel {
    fn get_harvest_level() -> f32;
}

pub trait Tool: Item + HasHarvestLevel {
    type ToolType: ToolType;
}

impl<'s, B> Item for &'s B
where
    B: Item,
{
    fn id(&self) -> usize {
        (*self).id()
    }

    fn get_namespace(&self) -> NameSpaceRef<'s> {
        (*self).get_namespace()
    }
}
