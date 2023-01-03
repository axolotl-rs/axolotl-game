use axolotl_noise::minecraft::random::xoroshiro::rand_xoshiro::Xoroshiro128PlusPlus;
use axolotl_noise::minecraft::random::xoroshiro::MinecraftXoroshiro128;
use axolotl_noise::minecraft::MinecraftPerlin;
use rand::SeedableRng;

use axolotl_api::world_gen::noise::density::perlin::Perlin;
use axolotl_api::world_gen::noise::Noise;

#[derive(Debug, Clone)]
pub struct GameNoise {
    // TODO make MinecraftPerlin a Enum for different random generators
    pub perlin: MinecraftPerlin<MinecraftXoroshiro128>,
    pub settings: Noise,
}

impl Perlin for GameNoise {
    type Seed = [u8; 16];
    type Noise = Noise;

    fn new(random: Self::Seed, noise: Self::Noise) -> Self {
        let random = MinecraftXoroshiro128 {
            seed_low: i64::from_be_bytes(random[0..8].try_into().unwrap()),
            seed_high: i64::from_be_bytes(random[8..16].try_into().unwrap()),
            rand: Xoroshiro128PlusPlus::from_seed(random),
        };
        Self {
            perlin: MinecraftPerlin::new(noise.clone(), random),
            settings: noise,
        }
    }

    fn get_setting(&self) -> &Self::Noise {
        &self.settings
    }

    fn get(&self, x: f64, y: f64, z: f64) -> f64 {
        self.perlin.get_value(x, y, z, 0f64, 0f64)
    }
}
