use std::io::{Read, Write};

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::var_int::VarInt;
use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::login::client_bound::ClientBoundPluginRequest;
use crate::packets::login::server_bound::ServerBoundLoginPluginResponse;
use crate::Protocol::Java;
use crate::{define_id_fns, Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

#[derive(PacketImplDebug)]
pub struct ClientBoundPluginImpl;

impl Packet for ClientBoundPluginImpl {
    define_packet!(
        ClientBoundPluginRequest,
        0x04,
        Bound::ClientBound,
        Stage::Login,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.message_id.write(w)?;
        content.channel.write(w)?;
        w.write_all(&content.data)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let message_id = VarInt::read(r)?;
        let channel = String::read(r)?;
        let mut data = Vec::new();
        r.read_to_end(&mut data)?;
        Ok(ClientBoundPluginRequest {
            message_id,
            channel,
            data,
        })
    }
    fn read_with_length<R: Read>(
        r: &mut R,
        length: usize,
    ) -> Result<Self::Content, PacketReadError> {
        let message_id = VarInt::read(r)?;
        let channel = String::read(r)?;
        // This could possibly be slightly bigger than the actual length of the data, but it's not a big deal. At most 6 bytes.
        let mut data = Vec::with_capacity(length - (channel.len()));

        r.read_to_end(&mut data)?;
        Ok(ClientBoundPluginRequest {
            message_id,
            channel,
            data,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ServerBoundPluginResponseImpl;

impl Packet for ServerBoundPluginResponseImpl {
    type Content = ServerBoundLoginPluginResponse;

    define_id_fns!(2);

    fn bound() -> Bound {
        Bound::ServerBound
    }

    fn stage() -> Stage {
        Stage::Login
    }

    fn protocol() -> Protocol {
        Protocol::Java(761)
    }

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.message_id.write(w)?;
        content.successful.write(w)?;
        w.write_all(&content.data)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let message_id = VarInt::read(r)?;
        let successful = bool::read(r)?;
        let mut data = Vec::new();
        r.read_to_end(&mut data)?;
        Ok(ServerBoundLoginPluginResponse {
            message_id,
            successful,
            data,
        })
    }
    fn read_with_length<R: Read>(
        r: &mut R,
        length: usize,
    ) -> Result<Self::Content, PacketReadError> {
        let message_id = VarInt::read(r)?;
        let successful = bool::read(r)?;
        let mut data = Vec::with_capacity(length - 1);
        r.read_to_end(&mut data)?;
        Ok(ServerBoundLoginPluginResponse {
            message_id,
            successful,
            data,
        })
    }
}
