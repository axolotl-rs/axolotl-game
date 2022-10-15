use axolotl_types::OwnedNameSpaceKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: OwnedNameSpaceKey,
    pub count: i8,
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    #[serde(rename = "Damage")]
    pub damage: i32,
    #[serde(rename = "Unbreakable")]
    pub unbreakable: bool,
}
