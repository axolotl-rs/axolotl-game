pub mod login;
pub mod play;

macro_rules! define_group {
    ($name:ident {
      $($variant:ident: $typ:path),*
    } ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $($variant($typ)),*
        }
        $(
            impl From<$typ> for $name {
                fn from(v: $typ) -> Self {
                    Self::$variant(v)
                }
            }
        )*
        impl PacketContent for $name {}

    };
}

pub(crate) use define_group;
