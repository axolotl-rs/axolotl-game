#[macro_export]
macro_rules! namespace_with_color {
    ($namespace:literal, $key:literal, $color:expr) => {
        match $color {
            DyeColor::White => NameSpaceRef::new($namespace, concat!($key, "_white")),
            DyeColor::Red => NameSpaceRef::new($namespace, concat!($key, "_red")),
        }
    };
}



pub trait Color {
    fn color(&self) -> u32;
    fn id(&self) -> u8;
}
#[derive(Debug, Clone)]
pub enum DyeColor {
    White,
    Red,
    // TODO: Add rest of colors
}

impl Color for DyeColor {
    fn color(&self) -> u32 {
        match self {
            DyeColor::White => 0xFFFFFF,
            DyeColor::Red => 11546150,
        }
    }

    fn id(&self) -> u8 {
        match self {
            DyeColor::White => 0,
            DyeColor::Red => 14,
        }
    }
}
