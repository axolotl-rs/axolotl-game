use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};

#[derive(Debug, Clone)]
pub struct OnceCache {}

impl DensityFunction for OnceCache {
    type FunctionDefinition = ();

    fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, _state: &State) -> f64 {
        todo!()
    }
}
