use std::io::{Read, Write};

use log::warn;

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::var_int::VarInt;
use crate::data::{NBTOrByteArray, PacketDataType};
use crate::java::call_write;
use crate::packets::play::client::login::GameMode;
use crate::packets::play::client::LoginPacket;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Protocol, Stage};

#[derive(PacketImplDebug)]
pub struct ClientBoundLoginPacketImpl;

impl Packet for ClientBoundLoginPacketImpl {
    type Content = LoginPacket;
    crate::define_id_fns!(0x24);
    fn bound() -> Bound {
        Bound::ClientBound
    }
    fn stage() -> Stage {
        Stage::Play
    }
    fn protocol() -> Protocol {
        Java(761)
    }
    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        call_write!(
            w,
            content.id,
            content.is_hardcore,
            content.game_mode,
            content.previous_game_mode,
            content.dimension_names,
            content.registry_codec,
            content.dimension_type,
            content.dimension_name
        );
        w.write_all(content.hashed_seed.as_ref())?;
        call_write!(
            w,
            content.max_players,
            content.view_distance,
            content.simulation_distance,
            content.reduced_debug_info,
            content.enable_respawn_screen,
            content.is_debug,
            content.is_flat,
            false
        );

        Ok(())
    }
    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(LoginPacket {
            id: i32::read(r)?,
            is_hardcore: bool::read(r)?,
            game_mode: GameMode::read(r)?,
            previous_game_mode: i8::read(r)?,
            dimension_names: Vec::<String>::read(r)?,
            registry_codec: NBTOrByteArray::read(r)?,
            dimension_type: String::read(r)?,
            dimension_name: String::read(r)?,
            hashed_seed: {
                let mut seed = [0u8; 8];
                r.read_exact(&mut seed)?;
                seed
            },
            max_players: VarInt::read(r)?,
            view_distance: VarInt::read(r)?,
            simulation_distance: VarInt::read(r)?,
            reduced_debug_info: bool::read(r)?,
            enable_respawn_screen: bool::read(r)?,
            is_debug: bool::read(r)?,
            is_flat: bool::read(r)?,
            death_location: {
                if bool::read(r)? {
                    // TODO: Read death location
                    warn!("Death location is not supported yet");
                    None
                } else {
                    None
                }
            },
        })
    }
}
