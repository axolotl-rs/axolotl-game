use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use minecraft_protocol_macros::{PacketContentType, PacketEnum};

use crate::data::var_int::VarInt;
use crate::data::PacketDataType;
use crate::packets::define_group;
use crate::packets::play::{KeepAlive, PlayPing, PlayPluginMessage};
use crate::PacketContent;

define_group!(ServerBoundPlay {
    PlayerMove: ServerBoundMove,
    KeepAlive: KeepAlive,
    Ping: PlayPing,
    ClientInformation: ClientInformation,
    PluginMessage: PlayPluginMessage,
    ConfirmTeleport: ConfirmTeleport
});
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub enum ServerBoundMove {
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayerPositionAndRotation {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerRotation {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct ConfirmTeleport(pub VarInt);

impl From<VarInt> for ConfirmTeleport {
    fn from(id: VarInt) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType, PacketEnum)]
#[repr(i32)]
#[error("Invalid Main Hand {0}")]
#[packet_type(VarInt)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType, PacketEnum)]
#[repr(i32)]
#[error("Invalid Chat Mode {0}")]
#[packet_type(VarInt)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

bitflags! {
     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
     pub struct SkinParts: u8 {
        const CAPE = 0b0000_0001;
        const JACKET = 0b0000_0010;
        const LEFT_SLEEVE = 0b0000_0100;
        const RIGHT_SLEEVE = 0b0000_1000;
        const LEFT_PANTS_LEG = 0b0001_0000;
        const RIGHT_PANTS_LEG = 0b0010_0000;
        const HAT = 0b0100_0000;
    }
}

#[derive(Debug, Clone, PartialEq, PacketContentType)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: SkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}
