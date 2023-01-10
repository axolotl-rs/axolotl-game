pub mod handshake;
pub mod status;
pub mod v_761;

macro_rules! define_packet {
    ($content:ty, $id:literal, $bound:expr, $stage:expr, $protcol:expr) => {
        type Content = $content;

        crate::define_id_fns!($id);
        fn bound() -> Bound {
            $bound
        }
        fn stage() -> Stage {
            $stage
        }
        fn protocol() -> Protocol {
            $protcol
        }
    };
}

macro_rules! call_write {
    ($write:ident, $($data:expr),*) => {
        $(
            crate::data::PacketDataType::write($data, $write)?;
        )*
    };
}
pub(crate) use call_write;
pub(crate) use define_packet;
