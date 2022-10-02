use std::fmt;
use std::fmt::Formatter;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};

use crate::world_gen::noise::density::loading::FunctionArgument;
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};
use crate::world_gen::noise::density::perlin::Perlin;

#[derive(Debug, Clone)]
pub struct SplineFunction{
    pub spline: Spline,
}
impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'_,P> for SplineFunction{
    type FunctionDefinition = Spline;

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        //TODO load spline
        Self{
            spline: def,
        }
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spline {
    pub coordinate: NameSpaceKeyOrType<FunctionArgument>,
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Point {
    pub derivative: f64,
    pub location: f64,
    pub value: SplineOrConstant,

}

#[derive(Debug, Clone)]
pub enum SplineOrConstant {
    Spline(Box<Spline>),
    Constant(f64),
}

struct SplineOrConstantVisitor;

impl<'de> Visitor<'de> for SplineOrConstantVisitor {
    type Value = SplineOrConstant;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: Error {
        Ok(SplineOrConstant::Constant(v))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error {
        Ok(SplineOrConstant::Constant(v as f64))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        Ok(SplineOrConstant::Spline(Box::new(Deserialize::deserialize(
            serde::de::value::MapAccessDeserializer::new(&mut map),
        )?)))
    }
}

impl<'de> Deserialize<'de> for SplineOrConstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SplineOrConstantVisitor)
    }
}
