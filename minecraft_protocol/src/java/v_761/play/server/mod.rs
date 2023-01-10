use std::borrow::Cow;
use std::io::{Read, Write};

use minecraft_protocol_macros::PacketImplDebug;
pub use move_packet::*;

use crate::data::var_int::VarInt;
use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::java::v_761::new_type_struct_define_packet;
pub use crate::java::v_761::play::client::login::ClientBoundLoginPacketImpl;
use crate::packets::play::server::{
    ClientInformation, ConfirmTeleport, ServerBoundPlay, SkinParts,
};
use crate::packets::play::{KeepAlive, PlayPing, PlayPluginMessage};
use crate::PacketIO;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

mod move_packet;

minecraft_protocol_macros::define_io!(ServerBoundPlay {
    0x13 => {
        type_name: SetPlayerPosition
        g_var: PlayerMove
    },
    0x14 => {
        type_name: SetPlayerPositionAndRotation
        g_var: PlayerMove
    },
    0x15 => {
        type_name: SetPlayerRotation
        g_var: PlayerMove
    },
    0x1F => {
        type_name: PongPacket
        g_var: Ping
    },
    0x11 => {
        type_name: KeepAlivePacket
        g_var:  KeepAlive
    },
    0x07 => {
        type_name: ClientInformationImpl
        g_var:  ClientInformation
    },
    0x00 => {
        type_name: ConfirmTeleportImpl
        g_var:  ConfirmTeleport
    },
0x0C	 => {
        type_name: ServerBoundPluginMessageImpl
        g_var:  PluginMessage
    }

}
);

new_type_struct_define_packet!(
    PongPacket,
    PlayPing,
    0x1F,
    Bound::ServerBound,
    Stage::Play,
    Java(761),
    i32
);
new_type_struct_define_packet!(
    KeepAlivePacket,
    KeepAlive,
    0x11,
    Bound::ServerBound,
    Stage::Play,
    Java(761),
    i64
);
new_type_struct_define_packet!(
    ConfirmTeleportImpl,
    ConfirmTeleport,
    0x00,
    Bound::ServerBound,
    Stage::Play,
    Java(761),
    VarInt
);
#[derive(PacketImplDebug)]
pub struct ClientInformationImpl;
impl Packet for ClientInformationImpl {
    define_packet!(
        ClientInformation,
        0x07,
        Bound::ServerBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(_content: Self::Content, _w: &mut W) -> Result<(), PacketWriteError> {
        todo!()
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(ClientInformation {
            locale: PacketDataType::read(r)?,
            view_distance: PacketDataType::read(r)?,
            chat_mode: PacketDataType::read(r)?,
            chat_colors: PacketDataType::read(r)?,
            displayed_skin_parts: SkinParts::from_bits(PacketDataType::read(r)?).ok_or(
                PacketReadError::InvalidData(anyhow::anyhow!("Invalid skin parts")),
            )?,
            main_hand: PacketDataType::read(r)?,
            enable_text_filtering: PacketDataType::read(r)?,
            allow_server_listings: PacketDataType::read(r)?,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ServerBoundPluginMessageImpl;

impl Packet for ServerBoundPluginMessageImpl {
    define_packet!(
        PlayPluginMessage,
        0x0C,
        Bound::ServerBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        match content.id {
            Cow::Borrowed(borrow) => {
                PacketDataType::write(borrow, w)?;
            }
            Cow::Owned(own) => {
                PacketDataType::write(own, w)?;
            }
        }
        w.write_all(content.data.as_ref())?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        // Its going to be atleast 1 byte long
        Self::read_with_length(r, 1)
    }
    fn read_with_length<R: Read>(
        r: &mut R,
        length: usize,
    ) -> Result<Self::Content, PacketReadError> {
        let id = String::read(r)?;
        let mut data = Vec::<u8>::with_capacity(length - id.len());
        r.read_to_end(&mut data)?;
        Ok(PlayPluginMessage {
            id: Cow::Owned(id),
            data,
        })
    }
}
