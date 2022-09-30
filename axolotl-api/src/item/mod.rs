use crate::{NamespacedKey, NameSpaceRef};

pub mod block;
pub mod vanilla;

pub trait ToolType {
    fn name() -> &'static str;
}

pub trait Item {
    fn get_namespace(&self) -> NameSpaceRef<'static>;
}

pub trait HasHarvestLevel {
    fn get_harvest_level() -> f32;
}

pub trait Tool: Item + HasHarvestLevel {
    type ToolType: ToolType;
}

pub trait ItemRegistry {
    fn get_item(&self, name: impl NamespacedKey) -> Option<&Box<dyn Item>>;
}
