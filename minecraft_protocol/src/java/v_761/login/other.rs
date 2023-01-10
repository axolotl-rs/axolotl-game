use std::io::{Read, Write};

use uuid::Uuid;

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::var_int::{inline, VarInt};
use crate::data::{var_int, PacketDataType};
use crate::java::define_packet;
use crate::packets::login::client_bound::{Disconnect, LoginSuccess, SetCompression};
use crate::packets::login::Property;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

#[derive(PacketImplDebug)]
pub struct ClientBoundDisconnectImpl;

impl Packet for ClientBoundDisconnectImpl {
    type Content = Disconnect;
    crate::define_id_fns!(0);

    fn bound() -> Bound {
        Bound::ClientBound
    }

    fn stage() -> Stage {
        Stage::Login
    }

    fn protocol() -> Protocol {
        Java(761)
    }

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        debug_assert!(content.reason.len() <= 262144);
        Self::write_packet_id(w)?;
        content.reason.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let reason = String::read(r)?;
        Ok(Disconnect { reason })
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundLoginSuccessImpl;

impl Packet for ClientBoundLoginSuccessImpl {
    define_packet!(
        LoginSuccess,
        0x02,
        Bound::ClientBound,
        Stage::Login,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.uuid.write(w)?;
        content.username.write(w)?;
        let i = content.properties.len() as i32;
        var_int::inline::write(i, w)?;
        for property in content.properties {
            property.name.write(w)?;
            property.value.write(w)?;
            if let Some(signature) = property.signature {
                true.write(w)?;
                signature.write(w)?;
            } else {
                false.write(w)?;
            }
        }
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let uuid = Uuid::read(r)?;
        let username = String::read(r)?;
        let properties_len = inline::read(r)?.0;
        let mut properties = Vec::with_capacity(properties_len as usize);
        for _ in 0..properties_len {
            let name = String::read(r)?;
            let value = String::read(r)?;
            let signature = if bool::read(r)? {
                Some(String::read(r)?)
            } else {
                None
            };
            properties.push(Property {
                name,
                value,
                signature,
            });
        }
        Ok(LoginSuccess {
            uuid,
            username,
            properties,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundSetCompressionImpl;

impl Packet for ClientBoundSetCompressionImpl {
    type Content = SetCompression;

    crate::define_id_fns!(3);

    fn bound() -> Bound {
        Bound::ClientBound
    }

    fn stage() -> Stage {
        Stage::Login
    }

    fn protocol() -> Protocol {
        Java(761)
    }

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.0.write(w)?;
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let threshold = VarInt::read(r)?;
        Ok(SetCompression(threshold))
    }
}
