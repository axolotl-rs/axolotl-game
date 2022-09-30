use noise::{NoiseFn, Perlin as ImplNoise};
use rand::Rng;

use axolotl_api::world_gen::noise::density::perlin::Perlin as AxolotlPerlin;
use axolotl_api::world_gen::noise::NoiseSetting;

#[derive(Debug, Clone)]
pub struct GameNoise {
    pub perlin: ImplNoise,
    pub settings: NoiseSetting,
}

impl AxolotlPerlin for GameNoise {
    fn new(_random: impl Rng, noise: NoiseSetting) -> Self {
        Self {
            perlin: ImplNoise::new(),
            settings: noise,
        }
    }

    fn get_setting(&self) -> &NoiseSetting {
        todo!()
    }

    fn get(&self, x: f64, y: f64, z: f64) -> f64 {
        self.perlin.get([x, y, z])
    }
}
