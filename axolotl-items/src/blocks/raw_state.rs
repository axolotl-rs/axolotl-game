use std::fmt::Formatter;

use serde::de::{MapAccess, Visitor};
use serde::Deserialize;

use crate::blocks::generic_block::VanillaState;

#[derive(Debug)]
pub struct RawState {
    pub states: Vec<VanillaState>,
}
pub struct RawStateVisitor;

impl<'de> Visitor<'de> for RawStateVisitor {
    type Value = RawState;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("VanillaState")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            if key.eq("states") {
                values = map.next_value()?;
            } else {
                map.next_value::<serde_json::Value>()?;
            }
        }
        Ok(RawState { states: values })
    }
}

impl<'de> Deserialize<'de> for RawState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RawStateVisitor)
    }
}
