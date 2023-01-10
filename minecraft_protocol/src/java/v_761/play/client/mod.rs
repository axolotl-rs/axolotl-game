use std::borrow::Cow;
use std::io::{Read, Write};

use minecraft_protocol_macros::{define_io, PacketImplDebug};

use crate::data::{var_int, PacketDataType};
use crate::java::define_packet;
use crate::java::v_761::new_type_struct_define_packet;
use crate::java::v_761::play::client::chunk::{
    ClientBoundChunkDataImpl, ClientBoundLightUpdateImpl,
};
pub use crate::java::v_761::play::client::login::ClientBoundLoginPacketImpl;
use crate::java::v_761::play::client::player_info::SyncPlayerPositionImpl;
use crate::packets::play::client::{
    AbilitiesPacket, AbilityFlags, ChangeDifficultyPacket, ClientBoundPlay, DisconnectPacket,
};
use crate::packets::play::{KeepAlive, PlayPing, PlayPluginMessage};
use crate::PacketIO;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

pub mod chunk;
pub mod login;
mod player_info;

define_io!(ClientBoundPlay {
    0x24 => {
        type_name: ClientBoundLoginPacketImpl
        g_var:  Login
    },
    0x15 => {
        type_name: ClientBoundPluginMessageImpl
        g_var:  PluginMessage
    },
    0x30 => {
        type_name: ClientBoundSetAbilities
        g_var:  Abilities
    },
    0x0B => {
        type_name: ClientBoundChangeDifficulty
        g_var:  ChangeDifficulty
    },
    0x17 => {
        type_name: ClientBoundDisconnectPacketImpl
        g_var:  Disconnect
    },
    0x1F => {
        type_name: ClientBoundKeepAliveImpl
        g_var:  KeepAlive
    },
    0x2E => {
        type_name: ClientBoundPingImpl
        g_var:  Ping
    },
    0x38 => {
        type_name: SyncPlayerPositionImpl
        g_var:  SyncPlayerPosition
    },
    0x20 => {
        type_name: ClientBoundChunkDataImpl
        g_var:  ChunkData
    },
    0x23 => {
        type_name: ClientBoundLightUpdateImpl
        g_var:  UpdateLight
    }
});
new_type_struct_define_packet!(
    ClientBoundPingImpl,
    PlayPing,
    0x2E,
    Bound::ClientBound,
    Stage::Play,
    Java(761),
    i32
);
new_type_struct_define_packet!(
    ClientBoundKeepAliveImpl,
    KeepAlive,
    0x1F,
    Bound::ClientBound,
    Stage::Play,
    Java(761),
    i64
);

#[derive(PacketImplDebug)]
pub struct ClientBoundDisconnectPacketImpl;
impl Packet for ClientBoundDisconnectPacketImpl {
    define_packet!(
        DisconnectPacket,
        0x17,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        var_int::inline::write(content.0.len() as i32, w)?;
        w.write_all(content.0.as_bytes())?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let len = var_int::inline::read(r)?;
        let mut buf = Vec::with_capacity(len.0 as usize);
        r.read_exact(&mut buf)?;
        Ok(DisconnectPacket(String::from_utf8(buf)?))
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundPluginMessageImpl;

impl Packet for ClientBoundPluginMessageImpl {
    define_packet!(
        PlayPluginMessage,
        0x15,
        Bound::ClientBound,
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

#[derive(PacketImplDebug)]
pub struct ClientBoundChangeDifficulty;

impl Packet for ClientBoundChangeDifficulty {
    define_packet!(
        ChangeDifficultyPacket,
        0x0B,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        (content.difficulty as u8).write(w)?;
        content.locked.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let difficulty = u8::read(r)?;
        let locked = bool::read(r)?;
        Ok(ChangeDifficultyPacket {
            difficulty: difficulty
                .try_into()
                .map_err(|x| PacketReadError::InvalidData(anyhow::Error::new(x)))?,
            locked,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundSetAbilities;

impl Packet for ClientBoundSetAbilities {
    define_packet!(
        AbilitiesPacket,
        0x30,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.flags.bits().write(w)?;
        content.flying_speed.write(w)?;
        content.walking_speed.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let flags = u8::read(r)?;
        let flying_speed = f32::read(r)?;
        let walking_speed = f32::read(r)?;
        Ok(AbilitiesPacket {
            flags: AbilityFlags::from_bits(flags).ok_or(PacketReadError::InvalidData(
                anyhow::Error::msg("Invalid ability flags"),
            ))?,
            flying_speed,
            walking_speed,
        })
    }
}
