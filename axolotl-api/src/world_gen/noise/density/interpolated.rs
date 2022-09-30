use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};

///https://minecraft.fandom.com/wiki/Density_function#interpolated
#[derive(Debug, Clone)]
pub struct Interpolated {}

impl DensityFunction for Interpolated {
    type FunctionDefinition = ();

    fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, _state: &State) -> f64 {
        todo!("Interpolated")
    }
}
