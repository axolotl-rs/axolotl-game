#![allow(unused)]
extern crate core;

pub mod item_stack;
mod registry;
pub mod world;

pub use flume::{Receiver, Sender};

pub struct ChunkPosSplit(i32, i32);

#[test]
pub fn test_build() {}
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
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    WorldError(#[from] axolotl_world::Error),
    #[error(transparent)]
    NbtError(#[from] axolotl_nbt::NBTError),
    #[error(transparent)]
    SerdeError(#[from] serde_impl::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

use crate::world::generator::AxolotlDensityLoader;
use crate::world::perlin::GameNoise;
use crate::world::AxolotlWorld;
use axolotl_api::game::{AxolotlVersion, DataRegistries, Game, Registries};

use axolotl_api::world_gen::biome::vanilla::DataPackBiome;

use axolotl_api::world_gen::noise::{Noise, NoiseSetting};
use axolotl_api::NamespacedKey;

use axolotl_nbt::serde_impl;
pub(crate) use get_type;
use log::{debug, info};
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

use crate::item_stack::AxolotlItemStack;
use axolotl_items::blocks::MinecraftBlock;
use axolotl_items::items::MinecraftItem;
use axolotl_world::level::MinecraftVersion;
use registry::SimpleRegistry;
use thiserror::Error;

const AXOLOTL_VERSION: &str = include_str!("axolotl-version.json");
const MINECRAFT_VERSION: &str = include_str!("minecraft_version.json");

#[derive(Debug)]
pub struct GameConfig {
    // Path to the Minecraft Data Dump
    pub data_dump: PathBuf,
    // Any data packs to load
    pub data_packs: Vec<PathBuf>,
    // The data from https://github.com/PrismarineJS/minecraft-data/
    pub axolotl_data: PathBuf,
}
pub struct AxolotlGame {
    pub data_registries: AxolotlDataRegistries,
    pub registries: AxolotlRegistries,
    pub density_loader: AxolotlDensityLoader,
    pub minecraft_version: MinecraftVersion,
    pub axolotl_version: AxolotlVersion,
}
impl AxolotlGame {
    pub fn load(config: GameConfig) -> Result<Self, Error> {
        let minecraft_version: MinecraftVersion =
            serde_json::from_str(MINECRAFT_VERSION).expect("Failed to parse minecraft version");
        let axolotl_version: AxolotlVersion =
            serde_json::from_str(AXOLOTL_VERSION).expect("Failed to parse axolotl version");
        info!("Loading Minecraft {:?}", minecraft_version);
        info!("Loading Axolotl {:?}", axolotl_version);

        debug!("Attempting to load the data dump at {:?}", config.data_dump);
        if !config.data_dump.exists() {
            return Err(Error::IO(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Data dump not found",
            )));
        }
        let data_registries = AxolotlDataRegistries {
            noises: SimpleRegistry::load_from_path(
                config
                    .data_dump
                    .join("reports")
                    .join("minecraft")
                    .join("worldgen")
                    .join("noise"),
            )?,
            noise_settings: SimpleRegistry::load_from_path(
                config
                    .data_dump
                    .join("reports")
                    .join("minecraft")
                    .join("worldgen")
                    .join("noise_settings"),
            )?,
        };
        let density_loader = AxolotlDensityLoader(SimpleRegistry::load_from_path(
            config
                .data_dump
                .join("reports")
                .join("minecraft")
                .join("worldgen")
                .join("density_function"),
        )?);
        let materials = axolotl_items::load_materials(config.axolotl_data.clone()).unwrap();
        let mut block_registry = SimpleRegistry::new();
        axolotl_items::load_blocks(
            config.axolotl_data,
            config.data_dump.clone(),
            &materials,
            &mut block_registry,
        )
        .unwrap();

        let registries = AxolotlRegistries {
            biomes: SimpleRegistry::load_from_path(
                config
                    .data_dump
                    .join("reports")
                    .join("minecraft")
                    .join("worldgen")
                    .join("biome"),
            )?,
            blocks: block_registry,
        };

        // TODO: Load Data Packs
        Ok(Self {
            data_registries,
            registries,
            density_loader,
            minecraft_version,
            axolotl_version,
        })
    }
    pub fn get_block(&self, _ey: impl NamespacedKey) -> Option<&MinecraftBlock<AxolotlGame>> {
        todo!("get_block")
    }
}
impl Debug for AxolotlGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AxolotlGame {:?} Minecraft Version {:?}",
            self.axolotl_version, self.minecraft_version
        )
    }
}
impl Game for AxolotlGame {
    type World = AxolotlWorld<'static>;
    type Biome = DataPackBiome;
    type Block = MinecraftBlock<Self>;
    type Item = MinecraftItem<Self>;
    type ItemStack = AxolotlItemStack;

    type DensityLoader = AxolotlDensityLoader;
    type Perlin = GameNoise;
    type Registries = AxolotlRegistries;
    type DataRegistries = AxolotlDataRegistries;

    fn registries(&self) -> &Self::Registries {
        &self.registries
    }

    fn mut_registries(&mut self) -> &mut Self::Registries {
        &mut self.registries
    }

    fn data_registries(&self) -> &Self::DataRegistries {
        &self.data_registries
    }

    fn mut_data_registries(&mut self) -> &mut Self::DataRegistries {
        &mut self.data_registries
    }
}
pub struct AxolotlRegistries {
    pub biomes: SimpleRegistry<DataPackBiome>,
    pub blocks: SimpleRegistry<MinecraftBlock<AxolotlGame>>,
}
impl Registries<AxolotlGame> for AxolotlRegistries {
    type BiomeRegistry = SimpleRegistry<DataPackBiome>;

    fn get_biome_registry(&self) -> &Self::BiomeRegistry {
        &self.biomes
    }

    fn get_mut_biome_registry(&mut self) -> &mut Self::BiomeRegistry {
        &mut self.biomes
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
