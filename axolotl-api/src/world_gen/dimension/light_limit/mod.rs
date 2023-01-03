use std::fmt::Formatter;

use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::world_gen::dimension::MonsterSpawnLightLevel;

pub struct MonsterSpawnLightLevelVisitor;
impl<'de> Visitor<'de> for MonsterSpawnLightLevelVisitor {
    type Value = MonsterSpawnLightLevel;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("An Integer or Struct")
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(MonsterSpawnLightLevel::Constant(v as i32))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(key_name) = map.next_key::<String>()? {
            if key_name == "type" {
                return match map.next_value::<String>()?.as_str() {
                    "minecraft:uniform" => {
                        map.next_key::<String>()?;
                        let uniform = map.next_value::<UniformValue>()?;
                        Ok(MonsterSpawnLightLevel::Uniform(uniform))
                    }
                    _ => Err(serde::de::Error::custom(
                        "Invalid type for MonsterSpawnLightLevel",
                    )),
                };
            }
        }
        Err(A::Error::custom("Expected a String"))
    }
}
impl<'de> Deserialize<'de> for MonsterSpawnLightLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(MonsterSpawnLightLevelVisitor)
    }
}
impl Serialize for MonsterSpawnLightLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MonsterSpawnLightLevel::Constant(v) => serializer.serialize_i32(*v),
            MonsterSpawnLightLevel::Uniform(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "minecraft:uniform")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniformValue {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}
