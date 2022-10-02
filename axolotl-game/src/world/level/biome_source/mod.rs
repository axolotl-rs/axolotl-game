use crate::get_type;
use axolotl_api::world_gen::noise::BiomeSource;
use axolotl_api::{NamespacedKey, OwnedNameSpaceKey};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
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

    fn new(preset: Self::Preset) -> Self {
        todo!()
    }
}
#[derive(Debug)]
pub enum BiomeSourceSettings {
    MultiNoise {},
    TheEnd {},
    Fixed { biome: OwnedNameSpaceKey },
    Checkerboard {},
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
            "multi_noise" => Ok(BiomeSourceSettings::MultiNoise {}),
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
        deserializer.deserialize_str(BiomeSourceSettingsVisitor)
    }
}
