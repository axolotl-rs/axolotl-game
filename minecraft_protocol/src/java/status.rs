use std::fmt::Debug;
use std::hash::Hash;
use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::PacketDataType;
use crate::{
    Bound, Packet, PacketContent, PacketIO, PacketReadError, PacketWriteError, Protocol, Stage,
};

#[derive(PacketImplDebug)]
pub struct StatusRequestImpl;

impl Packet for StatusRequestImpl {
    type Content = ();
    crate::define_id_fns!(0);

    fn bound() -> Bound {
        Bound::ServerBound
    }

    fn stage() -> Stage {
        Stage::Status
    }

    fn protocol() -> Protocol {
        crate::Protocol::Java(0)
    }
    #[inline(always)]
    fn write<W: Write>(_content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        Ok(())
    }
    #[inline(always)]
    fn read<R: Read>(_: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(())
    }
}

#[derive(PacketImplDebug)]
pub struct StatusPingRequestImpl;

impl Packet for StatusPingRequestImpl {
    type Content = i64;

    crate::define_id_fns!(1);

    fn bound() -> Bound {
        Bound::ServerBound
    }

    fn stage() -> Stage {
        Stage::Status
    }

    fn protocol() -> Protocol {
        crate::Protocol::Java(0)
    }
    #[inline(always)]
    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.write(w)?;
        Ok(())
    }
    #[inline(always)]
    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(i64::read(r)?)
    }
}

#[derive(PacketImplDebug)]
pub struct StatusPingResponseImpl;

impl Packet for StatusPingResponseImpl {
    type Content = i64;

    crate::define_id_fns!(1);

    fn bound() -> Bound {
        Bound::ClientBound
    }

    fn stage() -> Stage {
        Stage::Status
    }

    fn protocol() -> Protocol {
        crate::Protocol::Java(0)
    }
    #[inline(always)]
    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.write(w)?;
        Ok(())
    }
    #[inline(always)]
    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(i64::read(r)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<Player>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status<Desc> {
    pub version: Version,
    pub players: Players,
    pub description: Desc,
    pub favicon: String,
    #[serde(default, rename = "previewsChat")]
    pub previews_chat: bool,
    #[serde(default, rename = "enforcesSecureChat")]
    pub enforced_secure_chat: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusOrString<Desc> {
    Status(Status<Desc>),
    JsonString(String),
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq> PacketContent
    for StatusOrString<T>
{
}

#[derive(Default)]
pub struct StatusResponseImpl<Desc: Serialize + Debug + Clone + PartialEq + Eq>(PhantomData<Desc>);

impl<T: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq> Debug
    for StatusResponseImpl<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusResponseImpl").finish()
    }
}

impl<Desc: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq> Packet
    for StatusResponseImpl<Desc>
{
    type Content = StatusOrString<Desc>;

    crate::define_id_fns!(0);
    fn bound() -> Bound {
        Bound::ClientBound
    }

    fn stage() -> Stage {
        Stage::Status
    }

    fn protocol() -> Protocol {
        crate::Protocol::Java(0)
    }

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        match content {
            StatusOrString::Status(status) => {
                let json = serde_json::to_string(&status);
                match json {
                    Ok(json) => {
                        json.write(w)?;
                    }
                    Err(e) => {
                        return Err(PacketWriteError::Other(e.to_string()));
                    }
                }
            }
            StatusOrString::JsonString(json) => {
                json.write(w)?;
            }
        }
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let json = String::read(r)?;
        let status = serde_json::from_str(&json);
        match status {
            Ok(status) => Ok(StatusOrString::Status(status)),
            Err(_err) => Ok(StatusOrString::JsonString(json)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientBoundStatusPacket<Desc: Serialize + Debug + Clone + PartialEq + Eq> {
    Response(StatusOrString<Desc>),
    Ping(i64),
}

impl<Desc: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq> PacketContent
    for ClientBoundStatusPacket<Desc>
{
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServerBoundStatusPacket {
    Request,
    Ping(i64),
}

impl PacketContent for ServerBoundStatusPacket {}
#[derive(Debug)]
pub struct ServerBoundStatusIO;

impl PacketIO for ServerBoundStatusIO {
    type Type = ServerBoundStatusPacket;

    fn handle_read<R: Read>(
        packed_id: i32,
        len: usize,
        reader: &mut R,
    ) -> Result<Self::Type, PacketReadError> {
        match packed_id {
            0 => Ok(ServerBoundStatusPacket::Request),
            1 => {
                let packet = ServerBoundStatusPacket::Ping(
                    StatusPingRequestImpl::read_with_length(reader, len)?,
                );
                Ok(packet)
            }
            _ => Err(PacketReadError::UnknownPacketId(packed_id)),
        }
    }

    fn handle_write<W: Write>(packet: Self::Type, writer: &mut W) -> Result<(), PacketWriteError> {
        match packet {
            ServerBoundStatusPacket::Request => {
                StatusRequestImpl::write_packet_id(writer)?;
            }
            ServerBoundStatusPacket::Ping(v) => {
                StatusPingRequestImpl::write_packet_id(writer)?;
                v.write(writer)?;
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct ClientBoundStatusIO;

impl PacketIO for ClientBoundStatusIO {
    type Type = ClientBoundStatusPacket<Value>;

    fn handle_read<R: Read>(
        packed_id: i32,
        _len: usize,
        reader: &mut R,
    ) -> Result<Self::Type, PacketReadError> {
        match packed_id {
            0 => {
                let packet = StatusResponseImpl::read(reader)?;
                Ok(ClientBoundStatusPacket::Response(packet))
            }
            1 => {
                let packet = StatusPingResponseImpl::read(reader)?;
                Ok(ClientBoundStatusPacket::Ping(packet))
            }
            _ => Err(PacketReadError::UnknownPacketId(packed_id)),
        }
    }

    fn handle_write<W: Write>(packet: Self::Type, writer: &mut W) -> Result<(), PacketWriteError> {
        match packet {
            ClientBoundStatusPacket::Response(response) => {
                StatusResponseImpl::write(response, writer)?;
            }
            ClientBoundStatusPacket::Ping(v) => {
                StatusPingResponseImpl::write(v, writer)?;
            }
        }
        Ok(())
    }
}
