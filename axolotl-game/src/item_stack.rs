use crate::AxolotlGame;

use axolotl_api::item::ItemStack;
use axolotl_items::items::MinecraftItem;

#[derive(Debug, Clone, PartialEq)]
pub struct AxolotlItemStack {
    pub item: MinecraftItem<AxolotlGame>,
    pub count: u8,
}
impl ItemStack<AxolotlGame> for AxolotlItemStack {
    fn get_item(&self) -> &MinecraftItem<AxolotlGame> {
        &self.item
    }

    fn get_count(&self) -> u8 {
        self.count
    }

    fn set_count(&mut self, count: u8) {
        self.count = count;
    }
}
