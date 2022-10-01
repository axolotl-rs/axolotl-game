use std::fmt::Debug;

use crate::game::{DataRegistries, Game, Registry};
use crate::world_gen::dimension::Value;
use crate::world_gen::noise::density::{DensityFunction, DensityState};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};
#[derive(Debug, Clone)]
pub enum NoiseFunctions<P: Perlin<Noise = Noise, Seed = [u8;16]>> {
    Shift(Shift<P>),
    ShiftA(ShiftA<P>),
    ShiftB(ShiftB<P>),
}
impl<P: Perlin<Noise = Noise, Seed = [u8;16]>> NoiseFunction<P> for NoiseFunctions<P> {
    fn get_perlin(&self) -> &P {
        match self {
            NoiseFunctions::Shift(f) => f.get_perlin(),
            NoiseFunctions::ShiftA(f) => f.get_perlin(),
            NoiseFunctions::ShiftB(f) => f.get_perlin(),
        }
    }

    fn get_noise(&self) -> &Noise {
        match self {
            NoiseFunctions::Shift(f) => f.get_noise(),
            NoiseFunctions::ShiftA(f) => f.get_noise(),
            NoiseFunctions::ShiftB(f) => f.get_noise(),
        }
    }
}
impl<P: Perlin<Noise = Noise, Seed = [u8;16]>> DensityFunction for NoiseFunctions<P> {
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        todo!()
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        match self {
            NoiseFunctions::Shift(f) => DensityFunction::compute(f, state),
            NoiseFunctions::ShiftA(f) =>  DensityFunction::compute(f, state),
            NoiseFunctions::ShiftB(f) =>  DensityFunction::compute(f, state),
        }
    }
}



pub trait NoiseFunction<P: Perlin<Noise=Noise>>: Debug + DensityFunction {
    fn get_perlin(&self) -> &P;

    fn get_noise(&self) -> &Noise;

    #[inline(always)]
    fn compute(&self, x: f64, y: f64, z: f64) -> f64 {
        return self.get_perlin().get(x * 0.25, y * 0.25, z * 0.25) * 4.0;
    }
}



#[derive(Debug, Clone)]
pub struct ShiftB<P: Perlin> {
    perlin: P,
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction for ShiftB<P> {
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        let noise = match def {
            NameSpaceKeyOrType::NameSpaceKey(k) => {
                game.data_registries().get_noise_registry().get(k).unwrap().clone()
            }
            NameSpaceKeyOrType::Type(v) => {
                v
            }
        };
        let value = P::new(state.seed(), noise);
        Self {
            perlin: value
        }
    }
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x(), state.get_y(), 0.0)
    }
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> NoiseFunction<P> for ShiftB<P> {
    fn get_perlin(&self) -> &P {
        return &self.perlin;
    }

    fn get_noise(&self) -> &Noise {
        return self.perlin.get_setting();
    }
}


#[derive(Debug, Clone)]
pub struct ShiftA<P: Perlin> {
    perlin: P,
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction for ShiftA<P> {
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        let noise = match def {
            NameSpaceKeyOrType::NameSpaceKey(k) => {
                game.data_registries().get_noise_registry().get(k).unwrap().clone()
            }
            NameSpaceKeyOrType::Type(v) => {
                v
            }
        };
        let value = P::new(state.seed(), noise);
        Self {
            perlin: value
        }
    }
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x(), 0.0, state.get_z())
    }
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> NoiseFunction<P> for ShiftA<P> {
    fn get_perlin(&self) -> &P {
        return &self.perlin;
    }

    fn get_noise(&self) -> &Noise {
        return self.perlin.get_setting();
    }
}

#[derive(Debug, Clone)]
pub struct Shift<P: Perlin> {
    perlin: P,
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction for Shift<P> {
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        let noise = match def {
            NameSpaceKeyOrType::NameSpaceKey(k) => {
                game.data_registries().get_noise_registry().get(k).unwrap().clone()
            }
            NameSpaceKeyOrType::Type(v) => {
                v
            }
        };
        let value = P::new(state.seed(), noise);
        Self {
            perlin: value
        }
    }
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x(), state.get_y(), state.get_z())
    }
}

impl<P: Perlin<Noise=Noise, Seed=[u8; 16]>> NoiseFunction<P> for Shift<P> {
    fn get_perlin(&self) -> &P {
        return &self.perlin;
    }

    fn get_noise(&self) -> &Noise {
        return self.perlin.get_setting();
    }
}