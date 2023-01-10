use std::io::{Read, Write};

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::play::client::player_info::{SyncPlayerPosition, SyncPlayerPositionFlags};
use crate::Protocol;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Stage};

#[derive(PacketImplDebug)]
pub struct SyncPlayerPositionImpl;

impl Packet for SyncPlayerPositionImpl {
    define_packet!(
        SyncPlayerPosition,
        0x38,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.x.write(w)?;
        content.y.write(w)?;
        content.z.write(w)?;
        content.yaw.write(w)?;
        content.pitch.write(w)?;
        content.flags.bits().write(w)?;
        content.teleport_id.write(w)?;
        content.dismount_vehicle.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(SyncPlayerPosition {
            x: PacketDataType::read(r)?,
            y: PacketDataType::read(r)?,
            z: PacketDataType::read(r)?,
            yaw: PacketDataType::read(r)?,
            pitch: PacketDataType::read(r)?,
            flags: SyncPlayerPositionFlags::from_bits(PacketDataType::read(r)?).unwrap(),
            teleport_id: PacketDataType::read(r)?,
            dismount_vehicle: PacketDataType::read(r)?,
        })
    }
}
