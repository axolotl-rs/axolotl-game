use crate::blocks::MinecraftBlock;
use axolotl_api::events::{Event, EventHandler};
use axolotl_api::game::Game;
use axolotl_api::item::{Item, ItemLeftClick, ItemType};
use axolotl_api::{NamespacedId, NumericId};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockItem<G: Game> {
    pub block: MinecraftBlock<G>,
    pub id: usize,
}

impl<G: Game> ItemType for BlockItem<G> {}

impl<G: Game> NumericId for BlockItem<G> {
    fn id(&self) -> usize {
        self.id
    }
}

impl<G: Game> NamespacedId for BlockItem<G> {
    fn namespace(&self) -> &str {
        self.block.namespace()
    }

    fn key(&self) -> &str {
        self.block.key()
    }
}

impl<G: Game> EventHandler<ItemLeftClick<G>> for BlockItem<G> {
    fn handle(
        &self,
        _event: ItemLeftClick<G>,
    ) -> Result<<ItemLeftClick<G> as Event>::Result, <ItemLeftClick<G> as Event>::Error> {
        return Ok(());
    }
}

impl<G: Game> Item<G> for BlockItem<G> {}
