use std::borrow::Cow;

use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState, Function};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::Noise;

/// https://minecraft.fandom.com/wiki/Density_function#clamp
#[derive(Debug, Clone)]
pub struct Clamp<'function, P: Perlin<Noise = Noise, Seed = [u8;16]>> {
    pub min: f64,
    pub max: f64,
    pub input: Cow<'function, Function<'function, P>>,
}

impl<'function,P:Perlin<Noise = Noise, Seed = [u8;16]>> DensityFunction for Clamp<'function,P> {
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        todo!()
    }


    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        let input = self.input.compute(state);
        input.clamp(self.min, self.max)
    }
}
