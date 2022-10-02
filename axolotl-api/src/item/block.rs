use std::collections::HashMap;
use std::fmt::Debug;

use crate::item::Item;
use crate::world::{World, WorldLocation};

/// A Generic Block State Type
#[derive(Debug, Clone)]
pub enum BlockStateValue {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

pub trait BlockState: Debug {
    fn get(&self, name: &str) -> Option<&BlockStateValue>;
}

pub trait Block: Item {
    type BlockState: BlockState;
    type BlockRules: BlockRules;
}

pub trait BlockRules {
    type B;
    fn on_place<W: World>(b: Self::B, location: WorldLocation<W>);
}

#[derive(Debug)]
pub struct HashMapBasedBlockState {
    map: HashMap<String, BlockStateValue>,
}

impl HashMapBasedBlockState {
    pub fn new(map: HashMap<String, BlockStateValue>) -> Self {
        Self { map }
    }
}

#[cfg(test)]
pub mod example_block {
    use std::collections::HashMap;
    use std::fmt::Debug;

    use crate::color::DyeColor;
    use crate::item::block::{Block, BlockRules, BlockState, BlockStateValue};
    use crate::item::Item;
    use crate::world::{World, WorldLocation};
    use crate::{namespace_with_color, NameSpaceRef};

    #[derive(Debug, Clone, PartialEq)]
    pub enum BedPart {
        Head,
        Foot,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum BedFacing {
        North,
        South,
        East,
        West,
    }

    #[derive(Debug, Clone)]
    pub struct BedState {
        pub occupied: bool,
        pub part: BedPart,
        pub facing: BedFacing,
        pub map: HashMap<String, BlockStateValue>,
    }

    impl BlockState for BedState {
        // The default values will be found within the map in the natural state
        fn get(&self, name: &str) -> Option<&BlockStateValue> {
            self.map.get(name)
        }
    }

    pub struct Bed {
        pub state: BedState,
        pub color: DyeColor,
    }

    #[derive(Debug)]
    pub struct BedRules;

    impl BlockRules for BedRules {
        type B = Bed;
        fn on_place<W: World>(b: Bed, location: WorldLocation<W>) {
            if b.state.part == BedPart::Foot {
                // In the real game the it would be placed based on the direction the player is facing
                location.world.set_block(
                    (location.x as i32, location.y as i32, location.z as i32 + 1),
                    Bed {
                        state: BedState {
                            occupied: false,
                            part: BedPart::Head,
                            facing: b.state.facing,
                            map: b.state.map.clone(),
                        },
                        color: b.color,
                    },
                );
            }
        }
    }

    impl Item for Bed {
        fn get_namespace(&self) -> NameSpaceRef<'static> {
            namespace_with_color!("minecraft", "bed", self.color)
        }
    }

    impl Block for Bed {
        type BlockState = BedState;
        type BlockRules = BedRules;
    }
}
