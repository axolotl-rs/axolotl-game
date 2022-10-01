use std::fmt::Debug;

use rand::{Rng, SeedableRng};

pub use holder::NoiseHolder;

use crate::world_gen::noise::NoiseSetting;

mod holder;

pub trait Perlin: Debug + Clone {
    type Seed;
    type Noise;
    fn new(random: Self::Seed, noise: Self::Noise) -> Self;

    fn get_setting(&self) -> & Self::Noise;

    fn get(&self, x: f64, y: f64, z: f64) -> f64;
}
