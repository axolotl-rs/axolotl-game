use std::borrow::Cow;

use crate::game::Game;
use crate::world_gen::noise::density::loading::{get_constant, DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{BuildDefResult, DensityFunction, DensityState, Function};
use crate::world_gen::noise::Noise;
use crate::NamespacedKey;

/// https://minecraft.fandom.com/wiki/Density_function#clamp
#[derive(Debug, Clone)]
pub struct Clamp<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    pub min: f64,
    pub max: f64,
    pub input: Cow<'function, Function<'function, P>>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for Clamp<'function, P>
{
    type FunctionDefinition = (f64, f64, Box<FunctionArgument>);

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        let input = self.input.compute(state);
        input.clamp(self.min, self.max)
    }
    fn max(&self) -> f64 {
        self.max
    }
    fn min(&self) -> f64 {
        self.min
    }
    fn build_definition(
        value: FunctionArgument,
        _state: &mut impl DensityLoader,
    ) -> Result<Self::FunctionDefinition, BuildDefResult> {
        if let FunctionArgument::Function {
            name,
            mut arguments,
        } = value
        {
            if name.get_key().eq("clamp") {
                let min = get_constant!(arguments, "min");
                let max = get_constant!(arguments, "max");
                let input = arguments.remove("input").ok_or("Missing input argument")?;
                Ok((min, max, input))
            } else {
                Err(BuildDefResult::NotFound(FunctionArgument::Function {
                    name,
                    arguments,
                }))
            }
        } else {
            Err(BuildDefResult::NotFound(value))
        }
    }
}

/// https://minecraft.fandom.com/wiki/Density_function#clamp
#[derive(Debug, Clone)]
pub struct YClampedGradient {
    pub from_value: f64,
    pub to_value: f64,
    pub from_y: f64,
    pub to_y: f64,
}

impl<P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'_, P> for YClampedGradient {
    type FunctionDefinition = (f64, f64, f64, f64);

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        todo!("YClampedGradient")
    }

    fn build_definition(
        value: FunctionArgument,
        _state: &mut impl DensityLoader,
    ) -> Result<Self::FunctionDefinition, BuildDefResult> {
        if let FunctionArgument::Function {
            name,
            mut arguments,
        } = value
        {
            if name.get_key().eq("y_clamped_gradient") {
                let from_value = get_constant!(arguments, "from_value");
                let to_value = get_constant!(arguments, "to_value");
                let from_y = get_constant!(arguments, "from_y");
                let to_y = get_constant!(arguments, "to_y");
                Ok((from_value, to_value, from_y, to_y))
            } else {
                Err(BuildDefResult::NotFound(FunctionArgument::Function {
                    name,
                    arguments,
                }))
            }
        } else {
            Err(BuildDefResult::NotFound(value))
        }
    }
}
