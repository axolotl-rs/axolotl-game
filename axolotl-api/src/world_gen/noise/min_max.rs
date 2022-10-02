use std::fmt;
use std::fmt::{Formatter, Write};

use serde::de::{SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct MinMax {
    pub min: f64,
    pub max: f64,
}

impl TryFrom<Vec<f64>> for MinMax {
    type Error = ();
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }
        Ok(Self {
            min: value[0],
            max: value[1],
        })
    }
}

pub struct MinMaxVisitor;

impl<'de> Visitor<'de> for MinMaxVisitor {
    type Value = MinMax;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("an array of two values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut min_max = MinMax { min: 0.0, max: 0.0 };
        let mut i = 0;
        while let Some(value) = seq.next_element::<f64>()? {
            match i {
                0 => min_max.min = value,
                1 => min_max.max = value,
                _ => return Err(serde::de::Error::custom("expected an array of two values")),
            }
            i += 1;
        }
        if i != 2 {
            return Err(serde::de::Error::custom("expected an array of two values"));
        }
        Ok(min_max)
    }
}

impl<'de> Deserialize<'de> for MinMax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MinMaxVisitor)
    }
}
impl Serialize for MinMax {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.min)?;
        seq.serialize_element(&self.max)?;
        seq.end()
    }
}
