use std::borrow::Cow;

use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState, Function};

/// https://minecraft.fandom.com/wiki/Density_function#clamp
#[derive(Debug, Clone)]
pub struct Clamp<'function> {
    pub min: f64,
    pub max: f64,
    pub input: Cow<'function, Function<'function>>,
}

impl<'function> DensityFunction for Clamp<'function> {
    type FunctionDefinition = ();

    fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        let input = self.input.compute(state);
        input.clamp(self.min, self.max)
    }
}
