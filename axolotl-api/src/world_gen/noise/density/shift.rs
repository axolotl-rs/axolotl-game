use std::fmt::Debug;

use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::Noise;

pub trait NoiseFunction<P: Perlin>: Debug + DensityFunction {
    fn get_perlin(&self) -> &P;

    fn get_noise(&self) -> &Noise;

    fn compute(&self, x: f64, y: f64, z: f64) -> f64 {
        return self.get_perlin().get(x * 0.25, y * 0.25, z * 0.25) * 4.0;
    }
}
#[derive(Debug, Clone)]
pub struct ShiftB<P: Perlin> {
    perlin: P,
    noise: Noise,
}

impl<P: Perlin> DensityFunction for ShiftB<P> {
    type FunctionDefinition = ();

    fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x(), state.get_y(), 0.0)
    }
}

impl<P: Perlin> NoiseFunction<P> for ShiftB<P> {
    fn get_perlin(&self) -> &P {
        return &self.perlin;
    }

    fn get_noise(&self) -> &Noise {
        return &self.noise;
    }
}
