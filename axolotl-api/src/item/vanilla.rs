pub mod tool_types {
    use super::super::ToolType;

    // Defines the ToolTypes
    macro_rules! tool_types {
        ($($typ:ident, $name:literal),*) => {
            $(
                pub struct $typ;
                impl ToolType for $typ {
                    fn name() -> &'static str {
                        $name
                    }
                }
            )*
        };
    }
    tool_types!(Axe, "axe", Pickaxe, "pickaxe", Shovel, "shovel", Hoe, "hoe");
}

pub mod blocks {
    use crate::item::block::{Block, BlockRules, BlockState, BlockStateValue};
    use crate::item::Item;
    use crate::world::{World, WorldLocation};
    use crate::NameSpaceRef;

    /// For types that should never have a block state
    impl BlockState for () {
        fn get(&self, _: &str) -> Option<&BlockStateValue> {
            None
        }
    }

    impl BlockRules for () {
        type B = ();

        fn on_place<W: World>(_: (), _: WorldLocation<W>) {
            // Nothing will happen
        }
    }

    /// Air literally has no state
    pub struct Air;

    impl Item for Air {
        fn get_namespace(&self) -> NameSpaceRef<'static> {
            NameSpaceRef::new("minecraft", "air")
        }
    }

    impl Block for Air {
        type BlockState = ();
        type BlockRules = ();
    }
}
