use crate::blocks::MinecraftBlock;
use axolotl_api::item::{Item, ItemType};
use axolotl_api::{NamespacedId, NumericId};
#[derive(Debug, Clone, PartialEq)]
pub struct BlockItem {
    pub block: MinecraftBlock,
    pub id: usize,
}
impl ItemType for BlockItem {}

impl NumericId for BlockItem {
    fn id(&self) -> usize {
        self.id
    }
}
impl NamespacedId for BlockItem {
    fn namespace(&self) -> &str {
        self.block.namespace()
    }

    fn key(&self) -> &str {
        self.block.key()
    }
}

impl Item for BlockItem {}

#[derive(Debug)]
pub enum MinecraftItem {
    BlockItem(BlockItem),
}

impl ItemType for MinecraftItem {}

impl NumericId for MinecraftItem {
    fn id(&self) -> usize {
        match self {
            MinecraftItem::BlockItem(item) => item.id(),
        }
    }
}

impl Item for MinecraftItem {}
