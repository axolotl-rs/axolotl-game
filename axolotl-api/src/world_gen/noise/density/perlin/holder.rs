use crate::world_gen::noise::density::perlin::Perlin;

pub struct NoiseHolder<P: Perlin> {
    perlin: P,
}
