use std::sync::Arc;

use axolotl_api::events::{Event, EventHandler};
use axolotl_api::game::Game;
use axolotl_api::item::{Item, ItemLeftClick, ItemType};
use axolotl_api::NumericId;
use block_item::BlockItem;

pub mod block_item;

pub type MinecraftItem<G> = Arc<InnerMinecraftItem<G>>;

#[derive(Debug)]
pub enum InnerMinecraftItem<G: Game> {
    Air,
    BlockItem(BlockItem<G>),
}
impl<G: Game> PartialEq for InnerMinecraftItem<G> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InnerMinecraftItem::Air, InnerMinecraftItem::Air) => true,
            (InnerMinecraftItem::BlockItem(a), InnerMinecraftItem::BlockItem(b)) => a.id == b.id,
            _ => false,
        }
    }
}
impl<G: Game> ItemType for InnerMinecraftItem<G> {}

impl<G: Game> NumericId for InnerMinecraftItem<G> {
    fn id(&self) -> usize {
        match self {
            InnerMinecraftItem::BlockItem(item) => item.id(),
            InnerMinecraftItem::Air => 0,
        }
    }
}

impl<G: Game> EventHandler<ItemLeftClick<G>> for InnerMinecraftItem<G> {
    fn handle(
        &self,
        event: ItemLeftClick<G>,
    ) -> Result<<ItemLeftClick<G> as Event>::Result, <ItemLeftClick<G> as Event>::Error> {
        match self {
            InnerMinecraftItem::BlockItem(item) => item.handle(event),
            InnerMinecraftItem::Air => Ok(()),
        }
    }
}

impl<G: Game> Item<G> for InnerMinecraftItem<G> {}
