use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
#[cfg(feature= "tabled")]
use tabled::Tabled;
use crate::{NamespacedKey, OwnedNameSpaceKey};
use crate::world_gen::noise::{BiomeSource, Noise, NoiseSetting};
use crate::world_gen::noise::density::spline::Spline;

pub trait DensityLoader {
    type BiomeSource: BiomeSource;

    fn prep_for_load(&self, value: FunctionArgument) -> FunctionArgument;

    fn register_top_level(&mut self, key: OwnedNameSpaceKey, value: FunctionArgument);

    fn get_settings(&self, name: impl NamespacedKey) -> &NoiseSetting;

    fn get_biome_source(&self, name: impl NamespacedKey) -> &Self::BiomeSource;
}

#[cfg_attr(feature="tabled", derive(Tabled))]
#[derive(Debug, Clone)]
pub enum FunctionArgument {
    Function {
        name: OwnedNameSpaceKey,
        arguments: Box<FunctionArgument>,
    },
    TwoArgumentFunction {
        name: OwnedNameSpaceKey,
        arguments: (Box<FunctionArgument>, Box<FunctionArgument>),
    },
    NamespaceKey(OwnedNameSpaceKey),
    Constant(f64),
    Spline(Box<Spline>),
}


struct FunctionArgumentVisitor;

impl<'de> Visitor<'de> for FunctionArgumentVisitor {
    type Value = FunctionArgument;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("map, string, or number")
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::Constant(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::Constant(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::NamespaceKey(OwnedNameSpaceKey::from_str(v).map_err(|_|E::custom("Invalid namespace key"))?))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(FunctionArgument::NamespaceKey(OwnedNameSpaceKey::from_str(&v).map_err(|_|E::custom("Invalid namespace key"))?))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        let key = map.next_key::<String>()?;
        if let Some(key) = key.as_ref() {
            if key.eq("type") {
                let value = map.next_value::<OwnedNameSpaceKey>()?;
                let argument_one = map.next_key::<String>()?;
                if let Some(key) = argument_one {
                    if key.eq("argument") {
                        return Ok(FunctionArgument::Function {
                            name: value,
                            arguments: Box::new(map.next_value()?),
                        });
                    } else if key.eq("argument1") {
                        let argument_one = map.next_value::<FunctionArgument>()?;
                        let argument_two = map.next_entry::<String, FunctionArgument>()?;
                        return if let Some((key, argument_two)) = argument_two {
                            Ok(FunctionArgument::TwoArgumentFunction {
                                name: value,
                                arguments: (Box::new(argument_one), Box::new(argument_two)),
                            })
                        } else {
                            Err(A::Error::custom("Expected argument2"))
                        }
                    } else if key.eq("spline") {
                        return Ok(FunctionArgument::Spline(map.next_value()?));
                    }else{
                        return Ok(FunctionArgument::NamespaceKey(value));
                    }
                } else {
                    return Ok(FunctionArgument::NamespaceKey(value));
                }
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
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use tabled::Table;
    use crate::world_gen::noise::density::loading::FunctionArgument;

    #[test]
    pub fn test(){
        let string = read_to_string(PathBuf::from(r"C:\Users\wherk\Desktop\make_my_server\generated\reports\minecraft\worldgen\density_function\overworld\jaggedness.json")).unwrap();
        let data: FunctionArgument = serde_json::from_str(&string).unwrap();

        println!("{:#?}", data);
    }
}