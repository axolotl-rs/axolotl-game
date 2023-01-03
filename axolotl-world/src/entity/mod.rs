use std::collections::HashMap;
use std::fmt::Debug;

use axolotl_nbt::axolotl_nbt_macros::ListSerialize;
use axolotl_nbt::binary::binary_uuid::BinaryUUID;
use axolotl_nbt::value::NameLessValue;
use axolotl_types::{OwnedNameSpaceKey, RawPosition, RawRotation};
use serde_derive::{Deserialize, Serialize};

use crate::entity::effects::Effect;
use crate::entity::random_types::{Armor, ArmorDropChances};
use crate::region::file::RegionFileType;

pub mod effects;
pub mod player;
pub mod random_types;

#[derive(Debug, Clone, PartialEq, ListSerialize)]
pub struct EntityChunkLocation {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawEntities {
    pub data_version: i32,
    pub position: EntityChunkLocation,
    pub entities: Vec<Entity<GenericEntityType>>,
}

impl RegionFileType for RawEntities {
    #[inline(always)]
    fn get_path() -> &'static str
    where
        Self: Sized,
    {
        "entities"
    }

    fn get_xz(&self) -> (i32, i32) {
        (self.position.x, self.position.z)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericEntityType {
    pub id: OwnedNameSpaceKey,
    pub other: HashMap<String, NameLessValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Entity<EntityType> {
    pub air: i16,
    pub custom_name: Option<String>,
    pub custom_name_visible: bool,
    pub fall_distance: f32,
    pub fire: i16,
    pub glowing: bool,
    pub has_visual_fire: bool,
    pub invulnerable: bool,
    pub motion: [f64; 3],
    pub no_gravity: bool,
    pub on_ground: bool,
    pub passengers: Vec<Entity<GenericEntityType>>,
    pub portal_cooldown: i32,
    pub pos: RawPosition,
    pub rotation: RawRotation,
    #[serde(default)]
    pub silent: bool,
    pub tags: Vec<String>,
    pub ticks_frozen: i32,
    #[serde(rename = "UUID")]
    pub uuid: BinaryUUID,
    #[serde(flatten)]
    pub entity_specific_properties: EntityType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModEntity {
    pub absorption_amount: f32,
    pub active_effects: Vec<Effect>,
    pub armor_drop_chances: ArmorDropChances,
    pub armor_items: Armor,
}
