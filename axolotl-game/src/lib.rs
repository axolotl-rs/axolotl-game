pub mod world;
#[test]
pub fn test_build() {
    println!("test_build");
}
macro_rules! get_type {
    ($map:expr) => {
        if let Some((key, value)) = $map.next_entry::<String, OwnedNameSpaceKey>()? {
            if key.eq("type") {
                value
            } else {
                return Err(serde::de::Error::custom(format!(
                    "Expected `type` key, got `{}`",
                    key
                )));
            }
        } else {
            return Err(serde::de::Error::custom("Expected `type` key, got nothing"));
        }
    };
}
use crate::world::block::MinecraftBlock;
use crate::world::generator::AxolotlDensityLoader;
use crate::world::perlin::GameNoise;
use crate::world::AxolotlWorld;
use axolotl_api::game::{DataRegistries, Game, Registries, Registry};
use axolotl_api::item::block::BlockState;
use axolotl_api::world_gen::biome::vanilla::DataPackBiome;
use axolotl_api::world_gen::noise::{Noise, NoiseSetting};
use axolotl_api::OwnedNameSpaceKey;
pub(crate) use get_type;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct AxolotlGame {
    pub registries: AxolotlDataRegistries,
    pub density_loader: AxolotlDensityLoader,
}
impl Debug for AxolotlGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AxolotlGame")
    }
}
impl Game for AxolotlGame {
    type World = AxolotlWorld<'static>;
    type Biome = DataPackBiome;
    type Block = &'static MinecraftBlock;

    type DensityLoader = AxolotlDensityLoader;
    type Perlin = GameNoise;
    type Registries = AxolotlRegistries;
    type DataRegistries = AxolotlDataRegistries;

    fn registries(&self) -> &Self::Registries {
        todo!()
    }

    fn mut_registries(&mut self) -> &mut Self::Registries {
        todo!()
    }

    fn data_registries(&self) -> &Self::DataRegistries {
        &self.registries
    }

    fn mut_data_registries(&mut self) -> &mut Self::DataRegistries {
        &mut self.registries
    }
}
pub struct AxolotlRegistries {}
impl AxolotlRegistries {
    pub fn new() -> Self {
        Self {}
    }
}
impl Registries<AxolotlGame> for AxolotlRegistries {
    type BlockRegistry = SimpleRegistry<&'static MinecraftBlock>;
    type BiomeRegistry = SimpleRegistry<DataPackBiome>;

    fn get_block_registry(&self) -> &Self::BlockRegistry {
        todo!()
    }

    fn get_biome_registry(&self) -> &Self::BiomeRegistry {
        todo!()
    }

    fn get_mut_block_registry(&mut self) -> &mut Self::BlockRegistry {
        todo!()
    }

    fn get_mut_biome_registry(&mut self) -> &mut Self::BiomeRegistry {
        todo!()
    }
}

pub struct AxolotlDataRegistries {
    pub noises: SimpleRegistry<Noise>,
    pub noise_settings: SimpleRegistry<NoiseSetting>,
}
impl DataRegistries for AxolotlDataRegistries {
    type NoiseRegistry = SimpleRegistry<Noise>;
    type NoiseSettingRegistry = SimpleRegistry<NoiseSetting>;

    fn get_noise_registry(&self) -> &Self::NoiseRegistry {
        &self.noises
    }

    fn get_noise_setting_registry(&self) -> &Self::NoiseSettingRegistry {
        &self.noise_settings
    }

    fn get_mut_noise_registry(&mut self) -> &mut Self::NoiseRegistry {
        &mut self.noises
    }

    fn get_mut_noise_setting_registry(&mut self) -> &mut Self::NoiseSettingRegistry {
        &mut self.noise_settings
    }
}

pub struct SimpleRegistry<T> {
    pub map: HashMap<OwnedNameSpaceKey, T>,
}
impl<T> SimpleRegistry<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
impl<T> Default for SimpleRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Registry<T> for SimpleRegistry<T> {
    fn register(&mut self, namespace: OwnedNameSpaceKey, item: T) {
        self.map.insert(namespace, item);
    }

    fn get(&self, key: &OwnedNameSpaceKey) -> Option<&T> {
        self.map.get(key)
    }
}
