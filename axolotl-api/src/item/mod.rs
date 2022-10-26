use crate::events::{Event, EventHandler};
use crate::game::Game;
use crate::{NameSpaceRef, NamespacedKey, NumericId};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

pub mod block;
pub mod recipes;
pub mod vanilla;

pub trait ItemStack<G: Game> {
    fn get_item(&self) -> &G::Item;
    fn get_count(&self) -> u8;
    fn set_count(&mut self, count: u8);
}

pub trait ItemType: Debug + Send + Sync {}
impl<IT: ItemType> ItemType for &'_ IT {}
impl<IT: ItemType> ItemType for Arc<IT> {}
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

pub trait Item<G: Game>: ItemType + NumericId + EventHandler<ItemLeftClick<G>> {}

pub trait HasHarvestLevel {
    fn get_harvest_level() -> f32;
}

pub trait Tool<G: Game>: Item<G> + HasHarvestLevel {
    type ToolType: ToolType;
}

impl<'s, B, G: Game> Item<G> for &'s B where B: Item<G> + NumericId + EventHandler<ItemLeftClick<G>> {}
impl<B, G: Game> Item<G> for Arc<B> where B: Item<G> + NumericId + EventHandler<ItemLeftClick<G>> {}
