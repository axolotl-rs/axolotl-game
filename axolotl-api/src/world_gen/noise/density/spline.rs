use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState, Function};
use serde::de::{DeserializeOwned, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Formatter;

use crate::world_gen::noise::density::loading::FunctionArgument;
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};

#[derive(Debug, Clone)]
pub enum SplineFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    Spline {
        function: Function<'function, P>,
        derivatives: f64,
        value: Vec<SplineOrConstant<SplineFunction<'function, P>>>,
        min: f64,
        max: f64,
    },
    Constant(f64),
}
impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for SplineFunction<'function, P>
{
    type FunctionDefinition = Spline;

    fn new<G, DS: DensityState>(game: &G, state: &DS, mut def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        let function = state.build_from_def_with_cache::<G, P>(game, def.coordinate);
        todo!("Spline")
    }
    #[inline]
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        match self {
            SplineFunction::Spline { .. } => {
                todo!("Spline")
            }
            SplineFunction::Constant(value) => *value,
        }
    }
    #[inline]

    fn min(&self) -> f64 {
        match self {
            SplineFunction::Spline { min, .. } => *min,
            SplineFunction::Constant(value) => *value,
        }
    }
    #[inline]
    fn max(&self) -> f64 {
        match self {
            SplineFunction::Spline { max, .. } => *max,
            SplineFunction::Constant(value) => *value,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spline {
    pub coordinate: NameSpaceKeyOrType<FunctionArgument>,
    pub points: Vec<Point<Spline>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Point<T> {
    pub derivative: f64,
    pub location: f64,
    pub value: SplineOrConstant<T>,
}

#[derive(Debug, Clone)]
pub enum SplineOrConstant<T> {
    Spline(Box<T>),
    Constant(f64),
}

struct SplineOrConstantVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for SplineOrConstantVisitor<T> {
    type Value = SplineOrConstant<T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SplineOrConstant::Constant(v))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SplineOrConstant::Constant(v as f64))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(SplineOrConstant::Spline(Box::new(
            Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(&mut map))?,
        )))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SplineOrConstant<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SplineOrConstantVisitor(std::marker::PhantomData))
    }
}
