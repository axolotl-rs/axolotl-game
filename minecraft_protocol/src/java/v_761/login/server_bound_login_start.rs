use std::io::{Read, Write};

use uuid::Uuid;

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::login::server_bound::ServerBoundLoginStart;
use crate::packets::login::SigData;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

/// The 1.19.2 ServerBoundLoginStartImpl
///
/// Based on [wiki.vg](https://wiki.vg/index.php?title=Protocol&oldid=17829#Login_Start)
#[derive(PacketImplDebug)]
pub struct ServerBoundLoginStartImpl;

impl Packet for ServerBoundLoginStartImpl {
    define_packet!(
        ServerBoundLoginStart,
        0x00,
        Bound::ServerBound,
        Stage::Login,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.name.write(w)?;
        if let Some(uuid) = content.uuid {
            true.write(w)?;
            uuid.write(w)?;
        } else {
            false.write(w)?;
        }
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let name = String::read(r)?;
        let uuid = if bool::read(r)? {
            Some(Uuid::read(r)?)
        } else {
            None
        };
        Ok(ServerBoundLoginStart { name, uuid })
    }
}
