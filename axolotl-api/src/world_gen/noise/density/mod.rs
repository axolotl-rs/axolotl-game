use std::fmt::Debug;

use rand::Rng;
use serde_json::Value;

use crate::game::Game;
use crate::world_gen::noise::density::builtin::one_param::OneArgBuiltInFunction;
use crate::world_gen::noise::density::builtin::two_param::TwoParamBuiltInFunction;
use crate::world_gen::noise::density::cache::CacheFunctions;
use crate::world_gen::noise::density::clamp::Clamp;
use crate::world_gen::noise::density::groups::define_group_def;
use crate::world_gen::noise::density::interpolated::Interpolated;
use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::shift::NoiseFunctions;
use crate::world_gen::noise::density::spline::SplineFunction;
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};

pub mod builtin;
pub mod cache;
mod clamp;
pub mod groups;
mod interpolated;
pub mod loading;
pub mod perlin;
mod shift;
pub mod spline;

pub enum BuildDefResult {
    InvalidFormat,
    DescriptiveError(&'static str),
    NotFound(FunctionArgument),
}

impl From<&'static str> for BuildDefResult {
    fn from(s: &'static str) -> Self {
        BuildDefResult::DescriptiveError(s)
    }
}

/// The Current Density State

pub trait DensityState {
    type Random: Rng;
    type Perlin: Perlin<Noise = Noise, Seed = [u8; 16]>;
    fn seed(&self) -> [u8; 16];

    fn get_random(&self) -> Self::Random;

    fn get_x(&self) -> i64;

    fn get_y(&self) -> i64;

    fn get_z(&self) -> i64;

    fn get_perlin(&self) -> &Self::Perlin;

    fn build_from_def<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: FunctionArgument,
    ) -> Function<P>;

    fn build_from_def_with_cache<G: Game, P: Perlin<Noise = Noise, Seed = [u8; 16]>>(
        &self,
        game: &G,
        def: NameSpaceKeyOrType<FunctionArgument>,
    ) -> Function<P>;
}

/// The DensityFunction is a generic trait for all density functions.
///
/// You pass in a DensityState which contains all the functions and noises that are available.
///
pub trait DensityFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>>:
    Debug + Clone
{
    type FunctionDefinition;

    fn new<G, DS: DensityState<Perlin = P>>(
        game: &G,
        state: &'function DS,
        def: Self::FunctionDefinition,
    ) -> Self
    where
        G: Game;
    fn compute<State: DensityState>(&self, state: &State) -> f64;
    /// The maximum value that this function can return.
    fn max(&self) -> f64 {
        f64::MAX
    }
    /// The minimum value that this function can return.
    fn min(&self) -> f64 {
        f64::MIN
    }

    fn build_definition(
        value: FunctionArgument,
        _state: &mut impl DensityLoader,
    ) -> Result<Self::FunctionDefinition, BuildDefResult> {
        Err(BuildDefResult::NotFound(value))
    }
}

#[derive(Debug, Clone)]
pub struct Constant(f64);

/// A Function is a wrapper around a DensityFunction.
impl<P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'_, P> for Constant {
    type FunctionDefinition = f64;

    fn new<G, DS: DensityState>(_: &G, _: &DS, def: Self::FunctionDefinition) -> Self {
        Self(def)
    }

    fn compute<State: DensityState>(&self, _: &State) -> f64 {
        self.0
    }
    fn max(&self) -> f64 {
        self.0
    }
    fn min(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum Function<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    /// A constant value
    Constant(f64),
    Clamp(Box<Clamp<'function, P>>),
    Cached(Box<CacheFunctions<'function, P>>),
    OneParam(Box<OneArgBuiltInFunction<'function, P>>),
    TwoParam(Box<TwoParamBuiltInFunction<'function, P>>),
    Noise(Box<NoiseFunctions<'function, P>>),
    Spline(Box<SplineFunction<'function, P>>),
    Interpolated(Box<Interpolated<'function, P>>),
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for Function<'function, P>
{
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        panic!("This should never be called")
    }

    #[inline]
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.compute(state),
            Function::OneParam(builtin) => builtin.compute(state),
            Function::TwoParam(builtin) => builtin.compute(state),
            Function::Clamp(fun) => fun.compute(state),
            Function::Noise(value) => value.compute(state),
            Function::Cached(cache) => cache.compute(state),
            Function::Spline(spline) => spline.compute(state),
        }
    }
    #[inline]
    fn max(&self) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.max(),
            Function::OneParam(builtin) => builtin.max(),
            Function::TwoParam(builtin) => builtin.max(),
            Function::Clamp(fun) => fun.max(),
            Function::Cached(cache) => cache.max(),
            Function::Spline(spline) => spline.max(),
            Function::Noise(value) => value.max(),
        }
    }
    #[inline]
    fn min(&self) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.min(),
            Function::OneParam(builtin) => builtin.min(),
            Function::TwoParam(builtin) => builtin.min(),
            Function::Clamp(fun) => fun.min(),
            Function::Cached(cache) => cache.min(),
            Function::Spline(spline) => spline.min(),
            Function::Noise(value) => value.min(),
        }
    }
}
