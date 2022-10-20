use crate::level::{DataPacks, LevelDat, MinecraftVersion};
use crate::world::axolotl::AxolotlWorldError;
use axolotl_nbt::serde_impl;
use axolotl_nbt::value::{NameLessValue, Value};
use axolotl_types::OwnedNameSpaceKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxolotlDimension {
    pub dimension: OwnedNameSpaceKey,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AxolotlLevelDat {
    #[serde(rename = "Axolotl.Version")]
    pub axolotl_version: String,
    #[serde(rename = "Axolotl.PlayerData")]
    pub axolotl_player_data: String,
    pub version: MinecraftVersion,
    pub game_rules: Vec<Value>,
    pub data_packs: DataPacks,

    #[serde(rename = "version")]
    pub version_num: i32,
    pub data_version: i32,
    pub was_modded: bool,

    pub spawn_angle: f32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub level_name: String,

    #[serde(flatten)]
    pub other: HashMap<String, NameLessValue>,
}

impl TryFrom<LevelDat> for AxolotlLevelDat {
    type Error = AxolotlWorldError;

    fn try_from(mut value: LevelDat) -> Result<Self, Self::Error> {
        let axol_version = if let NameLessValue::String(v) =
            value
                .other
                .remove("Axolotl.Version")
                .ok_or(AxolotlWorldError::MissingAxolotlParam("Axolotl.version"))?
        {
            v
        } else {
            return Err(AxolotlWorldError::MissingAxolotlParam("Axolotl.version"));
        };
        let player_data = if let NameLessValue::String(v) = value
            .other
            .remove("Axolotl.PlayerData")
            .ok_or(AxolotlWorldError::MissingAxolotlParam("Axolotl.PlayerData"))?
        {
            v
        } else {
            return Err(AxolotlWorldError::MissingAxolotlParam("Axolotl.PlayerData"));
        };
        Ok(Self {
            axolotl_version: axol_version,
            axolotl_player_data: player_data,
            version: value.version,
            game_rules: value.game_rules,
            data_packs: value.data_packs,
            version_num: value.version_num,
            data_version: value.data_version,
            was_modded: value.was_modded,
            spawn_angle: value.spawn_angle,
            spawn_x: value.spawn_x,
            spawn_y: value.spawn_y,
            spawn_z: value.spawn_z,
            level_name: value.level_name,
            other: value.other,
        })
    }
}
