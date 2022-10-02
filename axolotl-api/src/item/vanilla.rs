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
