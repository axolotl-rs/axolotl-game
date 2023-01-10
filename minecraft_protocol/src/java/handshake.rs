use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::var_int::VarInt;
use crate::data::{var_int, PacketDataType};
use crate::{
    Bound, Packet, PacketContent, PacketIO, PacketReadError, PacketWriteError, Protocol, Stage,
};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum NextState {
    Status = 1,
    Login = 2,
}
impl PacketDataType for NextState {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        match u8::read(reader)? {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid NextState",
            )),
        }
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()> {
        var_int::inline::write(self as u8, writer).map(|_| ())
    }
}
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct HandShake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

impl PacketContent for HandShake {}

#[derive(PacketImplDebug)]
pub struct HandShakeImpl;

impl Packet for HandShakeImpl {
    type Content = HandShake;

    fn write_packet_id<W: Write>(w: &mut W) -> Result<usize, PacketWriteError> {
        0u8.write(w)?;
        Ok(1)
    }

    fn packet_id() -> i32 {
        0
    }

    fn bound() -> Bound {
        Bound::ServerBound
    }

    fn stage() -> Stage {
        Stage::Handshake
    }

    // This Packet does not change between versions
    fn protocol() -> Protocol {
        Protocol::Java(0)
    }

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.protocol_version.write(w)?;
        content.server_address.write(w)?;
        content.server_port.write(w)?;
        content.next_state.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let protocol_version = VarInt::read(r)?;
        let server_address = String::read(r)?;
        let server_port = u16::read(r)?;
        let next_state = NextState::read(r)?;
        Ok(HandShake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
#[derive(Debug)]
pub struct HandShakeIO;
impl PacketIO for HandShakeIO {
    type Type = HandShake;

    fn handle_read<R: Read>(
        packed_id: i32,
        _len: usize,
        reader: &mut R,
    ) -> Result<Self::Type, PacketReadError> {
        if packed_id != 0x00 {
            Err(PacketReadError::UnknownPacketId(packed_id))
        } else {
            HandShakeImpl::read(reader)
        }
    }

    fn handle_write<W: Write>(packet: Self::Type, writer: &mut W) -> Result<(), PacketWriteError> {
        HandShakeImpl::write(packet, writer)
    }
}
