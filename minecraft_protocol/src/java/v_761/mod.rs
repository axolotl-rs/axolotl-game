pub mod login;
pub mod play;
pub mod processor;

macro_rules! new_type_struct_define_packet {
    ($name:ident, $packet_type:ty, $id:literal, $bound:expr, $stage:expr, $protcol:expr, $inner_type:ty) => {
        #[derive(PacketImplDebug)]
        pub struct $name;
        impl Packet for $name {
            define_packet!($packet_type, $id, $bound, $stage, $protcol);

            fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
                Self::write_packet_id(w)?;
                content.0.write(w).map_err(PacketWriteError::from)
            }

            fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
                let value = <$inner_type>::read(r)?;
                Ok(<$packet_type>::from(value))
            }
        }
    };
}

pub(crate) use new_type_struct_define_packet;
