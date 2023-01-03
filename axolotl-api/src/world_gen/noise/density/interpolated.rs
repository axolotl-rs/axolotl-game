use std::marker::PhantomData;

use crate::game::Game;
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{DensityContext, DensityFunction, DensityState};
use crate::world_gen::noise::Noise;

///https://minecraft.fandom.com/wiki/Density_function#interpolated
#[derive(Debug, Clone)]
pub struct Interpolated<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    pub phantom: PhantomData<&'function P>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for Interpolated<'function, P>
{
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
    where
        G: Game,
    {
        todo!()
    }

    fn compute(&self, state: &impl DensityContext) -> f64 {
        todo!("Interpolated")
    }
}
