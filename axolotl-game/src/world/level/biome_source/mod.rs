use crate::get_type;
use axolotl_api::world_gen::noise::BiomeSource;
use axolotl_api::{NamespacedKey, OwnedNameSpaceKey};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Formatter;

pub enum AxolotlBiomeSource {
    MultiNoise {},
    TheEnd {},
    Fixed {},
    Checkerboard {},
}

impl BiomeSource for AxolotlBiomeSource {
    type Preset = BiomeSourceSettings;

    fn new(_preset: Self::Preset) -> Self {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub enum BiomeSourceSettings {
    MultiNoise { preset: OwnedNameSpaceKey },
    TheEnd {},
    Fixed { biome: OwnedNameSpaceKey },
    Checkerboard {},
}
impl Serialize for BiomeSourceSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            BiomeSourceSettings::MultiNoise { preset } => {
                map.serialize_entry("type", "minecraft:multi_noise")?;
                map.serialize_entry("preset", preset)?;
            }
            BiomeSourceSettings::TheEnd {} => {
                map.serialize_entry("type", "minecraft:the_end")?;
            }
            BiomeSourceSettings::Fixed { biome } => {
                map.serialize_entry("type", "minecraft:fixed")?;
                map.serialize_entry("biome", biome)?;
            }
            BiomeSourceSettings::Checkerboard {} => {
                map.serialize_entry("type", "minecraft:checkerboard")?;
            }
        }
        map.end()
    }
}
struct BiomeSourceSettingsVisitor;

impl<'de> Visitor<'de> for BiomeSourceSettingsVisitor {
    type Value = BiomeSourceSettings;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let value = get_type!(map);
        match value.get_key() {
            "multi_noise" => {
                let preset = map.next_entry::<String, OwnedNameSpaceKey>()?;
                return match preset {
                    None => Err(serde::de::Error::missing_field("preset")),
                    Some((name, v)) => {
                        if name != "preset" {
                            return Err(serde::de::Error::custom(format!(
                                "expected preset, got {}",
                                name
                            )));
                        }
                        Ok(BiomeSourceSettings::MultiNoise { preset: v })
                    }
                };
            }
            "the_end" => Ok(BiomeSourceSettings::TheEnd {}),
            "checkerboard" => Ok(BiomeSourceSettings::Checkerboard {}),
            "fixed" => {
                let biome = map.next_value::<OwnedNameSpaceKey>()?;
                Ok(BiomeSourceSettings::Fixed { biome })
            }
            _ => {
                return Err(serde::de::Error::custom(format!("Expected `type` key to be `multi_noise`, `the_end` or `checkerboard`, got `{}`", value.get_key())));
            }
        }
    }
}

impl<'de> Deserialize<'de> for BiomeSourceSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BiomeSourceSettingsVisitor)
    }
}
