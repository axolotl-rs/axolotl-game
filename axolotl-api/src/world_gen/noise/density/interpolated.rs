use std::marker::PhantomData;
use crate::game::Game;
use crate::world_gen::noise::density::{DensityFunction, DensityState};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::Noise;

///https://minecraft.fandom.com/wiki/Density_function#interpolated
#[derive(Debug, Clone)]
pub struct Interpolated<P: Perlin<Noise=Noise, Seed=[u8; 16]>> {
    pub phantom: PhantomData<P>,
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'_,P>  for Interpolated<P> {
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        todo!()
    }


    fn compute<State: DensityState>(&self, _state: &State) -> f64 {
        todo!("Interpolated")
    }
}
