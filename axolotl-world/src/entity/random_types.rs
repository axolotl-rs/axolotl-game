use std::collections::HashMap;

use axolotl_nbt::axolotl_nbt_macros::ListSerialize;
use axolotl_nbt::value::Value;
use axolotl_types::OwnedNameSpaceKey;
use serde_derive::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brain {
    pub memories: HashMap<OwnedNameSpaceKey, Vec<Value>>,
}

#[derive(Debug, Clone, ListSerialize, PartialEq)]
pub struct ArmorDropChances {
    pub boots: f32,
    pub chestplate: f32,
    pub helmet: f32,
    pub leggings: f32,
}

#[derive(Debug, Clone, PartialEq, ListSerialize)]
pub struct Armor {
    pub boots: Item,
    pub chestplate: Item,
    pub helmet: Item,
    pub leggings: Item,
}

#[derive(Debug, Clone, PartialEq, ListSerialize)]
pub struct Hand {
    pub main_hand: Item,
    pub off_hand: Item,
}

#[derive(Debug, Clone, PartialEq, ListSerialize)]
pub struct HandDropChances {
    pub main_hand: f32,
    pub off_hand: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub name: String,
    pub base: f64,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Modifier {
    #[serde(rename = "UUID")]
    pub uuid: Vec<u8>,
    pub name: String,
    pub amount: f64,
    pub operation: i32,
}
