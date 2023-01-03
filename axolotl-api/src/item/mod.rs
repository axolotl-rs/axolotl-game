use std::fmt::Debug;

use auto_impl::auto_impl;

use crate::events::{Event, EventHandler};
use crate::game::Game;
use crate::{NamespacedKey, NumericId};

pub mod block;
pub mod recipes;
pub mod vanilla;
pub trait ItemStack<G: Game> {
    fn get_item(&self) -> &G::Item;
    fn get_count(&self) -> u8;
    fn set_count(&mut self, count: u8);
}
#[auto_impl(Arc, &)]
pub trait ItemType: Debug + Send + Sync {}
#[auto_impl(Arc, &)]
pub trait ToolType {
    fn name() -> &'static str;
}
pub struct ItemLeftClick<G: Game> {
    pub item: G::Item,
}
impl<G: Game> Event for ItemLeftClick<G> {
    type Error = ();
    type Result = ();

    fn get_name() -> &'static str {
        "ItemLeftClick"
    }
}
#[auto_impl(Arc, &)]
pub trait Item<G: Game>: ItemType + NumericId + EventHandler<ItemLeftClick<G>> {}

pub trait HasHarvestLevel {
    fn get_harvest_level() -> f32;
}

#[auto_impl(Arc, &)]
pub trait Tool<G: Game>: Item<G> + HasHarvestLevel {
    type ToolType: ToolType;
}
