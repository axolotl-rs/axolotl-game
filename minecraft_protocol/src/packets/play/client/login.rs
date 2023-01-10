use std::fmt::Debug;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use minecraft_protocol_macros::PacketEnum;

use crate::data::var_int::VarInt;
use crate::data::{NBTOrByteArray, PacketDataType};
use crate::PacketContent;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize, PacketEnum)]
#[repr(u8)]
#[packet_type(u8)]
#[error("Invalid Game Mode {0}")]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginPacket {
    pub id: i32,
    pub is_hardcore: bool,
    pub game_mode: GameMode,
    pub previous_game_mode: i8,
    pub dimension_names: Vec<String>,
    pub registry_codec: NBTOrByteArray,
    pub dimension_type: String,
    pub dimension_name: String,
    pub hashed_seed: [u8; 8],
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(String, i64)>,
}

impl PacketContent for LoginPacket {}

impl LoginPacket {
    /// Hashes a seed using SHA-256
    pub fn hash_seed(seed: i128) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_be_bytes());
        let array = hasher.finalize();

        let mut hashed_seed = [0u8; 8];
        hashed_seed.copy_from_slice(&array[..8]);
        hashed_seed
    }
}
