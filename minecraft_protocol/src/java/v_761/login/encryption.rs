use std::io::{Read, Write};

use minecraft_protocol_macros::{define_var_int, PacketImplDebug};

use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::login::client_bound::ClientBoundEncryptionRequest;
use crate::packets::login::server_bound::ServerBoundEncryptionResponse;

use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

#[derive(PacketImplDebug)]
pub struct ServerBoundEncryptionResponseImpl;

impl Packet for ServerBoundEncryptionResponseImpl {
    define_packet!(
        ServerBoundEncryptionResponse,
        1,
        Bound::ServerBound,
        Stage::Login,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.shared_secret.write(w)?;
        content.verify_token.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let shared_secret = Vec::<u8>::read(r)?;
        let verify_token = Vec::<u8>::read(r)?;
        Ok(ServerBoundEncryptionResponse {
            shared_secret,
            verify_token,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundEncryptionRequestImpl;

impl Packet for ClientBoundEncryptionRequestImpl {
    define_packet!(
        ClientBoundEncryptionRequest,
        0x01,
        Bound::ClientBound,
        Stage::Login,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        w.write_all(&define_var_int!(0))?;
        content.public_key.write(w)?;
        content.verify_token.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let server_id = String::read(r)?;
        let public_key = Vec::<u8>::read(r)?;
        let verify_token = Vec::<u8>::read(r)?;
        Ok(ClientBoundEncryptionRequest {
            server_id,
            public_key,
            verify_token,
        })
    }
}
