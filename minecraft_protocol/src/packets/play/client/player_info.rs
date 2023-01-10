use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::data::var_int::VarInt;
use crate::packets::login::Property;
use crate::PacketContent;

pub enum Action {
    AddPlayer {
        name: String,
        properties: Vec<Property>,
        gamemode: u8,
        ping: i32,
        display_name: Option<String>,
    },
    UpdateGamemode,
    UpdateLatency,
    UpdateDisplayName,
    RemovePlayer,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfo {
    pub players: Vec<PlayerInfoEntry>,
}
#[derive(Debug, Clone, PartialEq)]

pub struct PlayerInfoEntry {
    pub uuid: String,
    pub name: String,
}

impl PacketContent for PlayerInfo {}

bitflags! {
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub struct SyncPlayerPositionFlags: u8 {
        const X = 0b0000_0001;
        const Y = 0b0000_0010;
        const Z = 0b0000_0100;
        const Y_ROT = 0b0000_1000;
        const X_ROT = 0b0001_0000;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: SyncPlayerPositionFlags,
    pub teleport_id: VarInt,
    pub dismount_vehicle: bool,
}
impl PacketContent for SyncPlayerPosition {}
