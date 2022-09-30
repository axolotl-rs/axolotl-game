use std::fmt::Debug;

use rand::Rng;

pub use holder::NoiseHolder;

use crate::world_gen::noise::NoiseSetting;

mod holder;

pub trait Perlin: Debug + Clone {
    fn new(random: impl Rng, noise: NoiseSetting) -> Self;

    fn get_setting(&self) -> &NoiseSetting;

    fn get(&self, x: f64, y: f64, z: f64) -> f64;
}
