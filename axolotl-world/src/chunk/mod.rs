use crate::region::file::RegionFileType;

use axolotl_types::{BadNamespacedKeyError, OwnedNameSpaceKey};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;

use std::str::FromStr;

pub mod compact_array;

pub const BITS_PER_BLOCK: u8 = 15;
pub const MINIMUM_BITS_PER_BLOCK: u8 = 4;

pub const CHUNK_HEIGHT: u16 = 384;
pub const CHUNK_WIDTH: u8 = 16;

pub const SECTION_HEIGHT: u8 = 16;
pub const SECTION_WIDTH: u8 = 16;
pub const SECTION_LENGTH: u8 = 16;

pub const BLOCKS_PER_SECTION: u32 =
    SECTION_HEIGHT as u32 * SECTION_WIDTH as u32 * SECTION_LENGTH as u32;

#[derive(Debug, Clone)]
pub struct PaletteItem {
    pub name: OwnedNameSpaceKey,
    pub properties: HashMap<String, String>,
}

impl FromStr for PaletteItem {
    type Err = BadNamespacedKeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PaletteItem {
            name: s.parse()?,
            properties: Default::default(),
        })
    }
}

impl TryFrom<String> for PaletteItem {
    type Error = BadNamespacedKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(PaletteItem {
            name: value.parse()?,
            properties: Default::default(),
        })
    }
}

impl Serialize for PaletteItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name.to_string())
    }
}

pub struct PaletteVisitor;

impl<'de> Visitor<'de> for PaletteVisitor {
    type Value = PaletteItem;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string or object")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(PaletteItem::from_str(v).map_err(Error::custom)?)
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        PaletteItem::try_from(v).map_err(Error::custom)
    }
    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut wrapped_key = map.next_key::<String>()?;
        let mut name = None;
        let mut properties = HashMap::new();
        while let Some(key) = wrapped_key {
            match key.as_str() {
                "Name" => {
                    name = Some(map.next_value()?);
                }
                "Properties" => {
                    properties = map.next_value()?;
                }
                key => {
                    log::warn!("Unknown key in palette item: {}", key);
                    map.next_key::<()>()?;
                }
            }
            wrapped_key = map.next_key::<String>()?;
        }
        Ok(PaletteItem {
            name: name.ok_or(Error::missing_field("Name"))?,
            properties,
        })
    }
}

impl<'de> Deserialize<'de> for PaletteItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(PaletteVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawChunk {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "xPos")]
    pub x_pos: i32,
    #[serde(rename = "yPos")]
    pub y_pos: i32,
    #[serde(rename = "zPos")]
    pub z_pos: i32,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    #[serde(default)]
    pub sections: Vec<ChunkSection>,
    #[serde(rename = "Lights", default)]
    pub lights: Vec<Vec<i16>>,
}

impl RegionFileType for RawChunk {
    fn get_path() -> &'static str
    where
        Self: Sized,
    {
        "region"
    }

    fn get_xz(&self) -> (i32, i32) {
        (self.x_pos, self.z_pos)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkSection {
    #[serde(rename = "Y")]
    pub y_pos: i8,
    pub block_states: Option<BlockStates>,
    pub biomes: Option<Biomes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStates {
    pub data: Option<Vec<u64>>,
    #[serde(default)]
    pub palette: Vec<PaletteItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biomes {
    #[serde(default)]
    pub data: Vec<u64>,
    pub palette: Vec<PaletteItem>,
}
