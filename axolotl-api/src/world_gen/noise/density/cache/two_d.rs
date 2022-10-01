use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};

#[derive(Debug, Clone)]
pub struct TwoDCache {}

impl DensityFunction for TwoDCache {
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        todo!()
    }


    fn compute<State: DensityState>(&self, _state: &State) -> f64 {
        todo!()
    }
}
