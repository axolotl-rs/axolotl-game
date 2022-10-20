use crate::blocks::generic_block::VanillaState;
use crate::blocks::hard_coded::HardCodedBlock;
use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use axolotl_api::NameSpaceRef;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct WhiteBed;

impl HardCodedBlock for WhiteBed {}
impl Item for WhiteBed {
    fn id(&self) -> usize {
        90
    }

    fn get_namespace(&self) -> NameSpaceRef<'_> {
        NameSpaceRef::new("minecraft", "white_bed")
    }
}

impl Block for WhiteBed {
    type State = VanillaState;

    fn get_default_state(&self) -> Self::State {
        todo!()
    }
}
