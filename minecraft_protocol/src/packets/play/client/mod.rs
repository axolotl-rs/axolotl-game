use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use minecraft_protocol_macros::{PacketContentType, PacketEnum};

use crate::data::PacketDataType;
use crate::packets::define_group;
use crate::packets::play::client::chunk::{ChunkDataAndLight, UpdateLightPacket};
pub use crate::packets::play::client::login::LoginPacket;
use crate::packets::play::client::player_info::{PlayerInfo, SyncPlayerPosition};
use crate::packets::play::{KeepAlive, PlayPing, PlayPluginMessage};
use crate::PacketContent;

pub mod chunk;
pub mod login;
pub mod player_info;

define_group!(ClientBoundPlay {
    Login: LoginPacket,
    Disconnect: DisconnectPacket,
    ServerData: ServerData,
    PluginMessage: PlayPluginMessage,
    Abilities: AbilitiesPacket,
    ChangeDifficulty: ChangeDifficultyPacket,
    KeepAlive: KeepAlive,
    Ping: PlayPing,
    SyncPlayerPosition: SyncPlayerPosition,
    PlayerInfo: PlayerInfo,
    ChunkData: ChunkDataAndLight,
    UpdateLight: UpdateLightPacket
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct DisconnectPacket(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerData {
    pub motd: Option<String>,
    pub icon: Option<String>,
    pub previews_chat: bool,
    pub enforced_secure_chat: bool,
}

impl PacketContent for ServerData {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketEnum, PacketContentType)]
#[error("Invalid Disconnect Packet {0}")]
#[repr(u8)]
#[packet_type(u8)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}

bitflags! {
     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
     pub struct AbilityFlags: u8 {
            const INVULNERABLE = 0b0000_0001;
            const FLYING = 0b0000_0010;
            const ALLOW_FLYING = 0b0000_1000;
            const CREATIVE_MODE = 0b1000_000;
    }
}
#[derive(Debug, Clone, PartialEq, PacketContentType)]
pub struct AbilitiesPacket {
    pub flags: AbilityFlags,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct SelectedSlotPacket(pub i32);
