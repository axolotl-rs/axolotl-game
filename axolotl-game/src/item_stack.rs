use crate::AxolotlGame;

use axolotl_api::item::ItemStack;
use axolotl_api::world::World;
use axolotl_items::items::MinecraftItem;

#[derive(Debug, Clone, PartialEq)]
pub struct AxolotlItemStack<W: World> {
    pub item: MinecraftItem<AxolotlGame<W>>,
    pub count: u8,
}
impl<W: World> ItemStack<AxolotlGame<W>> for AxolotlItemStack<W> {
    fn get_item(&self) -> &MinecraftItem<AxolotlGame<W>> {
        &self.item
    }

    fn get_count(&self) -> u8 {
        self.count
    }

    fn set_count(&mut self, count: u8) {
        self.count = count;
    }
}
