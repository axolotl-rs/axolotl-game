use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
#[cfg(feature = "tabled")]
use tabled::Tabled;
use crate::{NamespacedKey, OwnedNameSpaceKey};
use crate::world_gen::noise::{BiomeSource, NameSpaceKeyOrType, Noise, NoiseSetting};
use crate::world_gen::noise::density::spline::Spline;

macro_rules! get_constant {
    ($arguments:ident, $name:literal) => {
        match $arguments.remove($name).ok_or(concat!("Missing key ", $name))?.as_ref() {
                    FunctionArgument::ConstantFloat(v) => {
                        *v
                    }
                    _ => {
                        return Err("Max argument must be a constant float".into());
                    }
        }
    };
}
pub(crate) use get_constant;

pub trait DensityLoader {
    type BiomeSource: BiomeSource;

    fn prep_for_load(&self, value: FunctionArgument) -> FunctionArgument;

    fn register_top_level(&mut self, key: OwnedNameSpaceKey, value: FunctionArgument);

    fn get_settings(&self, name: impl NamespacedKey) -> &NoiseSetting;

    fn get_biome_source(&self, name: impl NamespacedKey) -> &Self::BiomeSource;
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
#[derive(Debug, Clone)]
pub enum FunctionArgument {
    Function {
        name: OwnedNameSpaceKey,
        arguments: HashMap<String, Box<FunctionArgument>>,
    },
    Noise(NameSpaceKeyOrType<Noise>),
    NamespaceKey(OwnedNameSpaceKey),
    String(String),
    ConstantFloat(f64),
    ConstantInt(i64),
    Spline(Box<Spline>),
}

struct FunctionArgumentVisitor;

impl<'de> Visitor<'de> for FunctionArgumentVisitor {
    type Value = FunctionArgument;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("map, string, or number")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantInt(v as i64))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantFloat(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::ConstantFloat(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if let Ok(key) = OwnedNameSpaceKey::from_str(&v) {
            Ok(FunctionArgument::NamespaceKey(key))
        } else {
            Ok(FunctionArgument::String(v.to_string()))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        if let Ok(key) = OwnedNameSpaceKey::from_str(&v) {
            Ok(FunctionArgument::NamespaceKey(key))
        } else {
            Ok(FunctionArgument::String(v))
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        let key = map.next_key::<String>()?;
        if let Some(key) = key.as_ref() {
            if key.eq("type") {
                let value = map.next_value::<OwnedNameSpaceKey>()?;
                let mut arguments = HashMap::with_capacity(map.size_hint().unwrap_or(0));
                while let Some(key) = map.next_key::<String>()? {
                    if key.eq("spline") {
                        let spline = map.next_value::<Spline>()?;
                        arguments.insert(key, Box::new(FunctionArgument::Spline(Box::new(spline))));
                    }else if key.eq("noise"){
                        let noise = map.next_value::<NameSpaceKeyOrType<Noise>>()?;
                        arguments.insert(key, Box::new(FunctionArgument::Noise(noise)));
                    } else {
                        arguments.insert(key, Box::new(map.next_value::<FunctionArgument>()?));
                    }
                }
                return Ok(FunctionArgument::Function { name: value, arguments });
            }
        }

        return Err(serde::de::Error::custom("Invalid function argument"));
    }
}

impl<'de> Deserialize<'de> for FunctionArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FunctionArgumentVisitor)
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs::{read_dir, read_to_string};
    use std::path::PathBuf;
    use tabled::Table;
    use crate::world_gen::noise::density::loading::FunctionArgument;

    #[test]
    pub fn test() {
        let buf = PathBuf::from(option_env!("DENSITY_FUNCTIONS").unwrap_or("density_functions"));
        if !buf.exists() {
            panic!("Density function directory does not exist");
        }
        read_folder(buf);
    }

    pub fn read_folder(path: PathBuf) {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                read_folder(entry.path());
            } else {
                println!("{}", entry.path().display());
                let contents = read_to_string(entry.path()).unwrap();
                match serde_json::from_str::<FunctionArgument>(&contents) {
                    Ok(value) => {
                        println!("Success");
                    }
                    Err(error) => {
                        println!("Failed to parse: {error:#?}");
                    }
                }
            }
        }
    }
}