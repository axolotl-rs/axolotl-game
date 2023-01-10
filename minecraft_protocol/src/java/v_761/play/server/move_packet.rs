use std::io::{Read, Write};

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::play::server::ServerBoundMove;
use crate::Protocol;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Stage};

#[derive(PacketImplDebug)]
pub struct SetPlayerPosition;

impl Packet for SetPlayerPosition {
    define_packet!(
        ServerBoundMove,
        0x13,
        Bound::ServerBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(_content: Self::Content, _w: &mut W) -> Result<(), PacketWriteError> {
        todo!("write")
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(ServerBoundMove::PlayerPosition {
            x: f64::read(r)?,
            y: f64::read(r)?,
            z: f64::read(r)?,
            on_ground: bool::read(r)?,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct SetPlayerPositionAndRotation;

impl Packet for SetPlayerPositionAndRotation {
    define_packet!(
        ServerBoundMove,
        0x14,
        Bound::ServerBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(_content: Self::Content, _w: &mut W) -> Result<(), PacketWriteError> {
        todo!("write")
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(ServerBoundMove::PlayerPositionAndRotation {
            x: f64::read(r)?,
            y: f64::read(r)?,
            z: f64::read(r)?,
            yaw: f32::read(r)?,
            pitch: f32::read(r)?,
            on_ground: bool::read(r)?,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct SetPlayerRotation;

impl Packet for SetPlayerRotation {
    define_packet!(
        ServerBoundMove,
        0x15,
        Bound::ServerBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(_content: Self::Content, _w: &mut W) -> Result<(), PacketWriteError> {
        todo!("write")
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(ServerBoundMove::PlayerRotation {
            yaw: f32::read(r)?,
            pitch: f32::read(r)?,
            on_ground: bool::read(r)?,
        })
    }
}
