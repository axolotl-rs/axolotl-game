#![allow(unused)]
extern crate core;

pub mod chat;
pub mod item_stack;
pub mod registry;
pub mod world;

pub use flume::{bounded, unbounded, Receiver, Sender};
use std::collections::VecDeque;

pub struct ChunkPosSplit(i32, i32);
/// Remove if https://github.com/zesterer/flume/issues/113 is fixed
pub struct FlumeHack<T> {
    pub queue: VecDeque<T>,
}
impl<T> FlumeHack<T> {
    pub fn from(drain: Drain<'_, T>) -> FlumeHack<T> {
        let value: FlumeHack<T> = unsafe { mem::transmute::<Drain<'_, T>, FlumeHack<T>>(drain) };
        value
    }
}
#[cfg(test)]
mod test {
    use flume::unbounded;

    #[test]
    pub fn test() {
        let (sender, receiver) = unbounded();
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();
        sender.send(4).unwrap();
        sender.send(5).unwrap();
        let drain = receiver.drain();
        let hack = super::FlumeHack::from(drain);

        let vec = hack.queue.into_iter().collect::<Vec<_>>();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
        println!("{:?}", vec);
    }
}
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
    AxolotlWorldError(#[from] axolotl_world::world::axolotl::AxolotlWorldError),
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
use flume::Drain;
pub(crate) use get_type;
use log::{debug, info};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::path::PathBuf;

use crate::chat::AxolotlChatType;
use crate::item_stack::AxolotlItemStack;
use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::dimension::Dimension;
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
            dimensions: SimpleRegistry::load_from_path(
                config
                    .data_dump
                    .join("reports")
                    .join("minecraft")
                    .join("dimension_type"),
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
        let chat_types = SimpleRegistry::load_from_path(
            config
                .data_dump
                .join("reports")
                .join("minecraft")
                .join("chat_type"),
        )?;
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
            chat_types,
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
    type World = AxolotlWorld;
    type Biome = DataPackBiome;
    type Block = MinecraftBlock<Self>;
    type Item = MinecraftItem<Self>;
    type ItemStack = AxolotlItemStack;

    type DensityLoader = AxolotlDensityLoader;
    type Registries = AxolotlRegistries;
    type DataRegistries = AxolotlDataRegistries;
    type ChatType = AxolotlChatType;

    fn create_placed_block(
        &self,
        block: Self::Block,
    ) -> <<Self as Game>::World as World>::WorldBlock {
        todo!("create_placed_block")
    }

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
    pub chat_types: SimpleRegistry<AxolotlChatType>,
}
impl Registries<AxolotlGame> for AxolotlRegistries {
    type BiomeRegistry = SimpleRegistry<DataPackBiome>;
    type BlockRegistry = SimpleRegistry<MinecraftBlock<AxolotlGame>>;
    type ItemRegistry = SimpleRegistry<MinecraftItem<AxolotlGame>>;
    type ChatTypeRegistry = SimpleRegistry<AxolotlChatType>;

    fn get_biome_registry(&self) -> &Self::BiomeRegistry {
        &self.biomes
    }

    fn get_block_registry(&self) -> &Self::BlockRegistry {
        &self.blocks
    }

    fn get_item_registry(&self) -> &Self::ItemRegistry {
        todo!()
    }

    fn get_chat_type_registry(&self) -> &Self::ChatTypeRegistry {
        &self.chat_types
    }

    fn get_mut_biome_registry(&mut self) -> &mut Self::BiomeRegistry {
        &mut self.biomes
    }

    fn get_mut_block_registry(&mut self) -> &mut Self::BlockRegistry {
        todo!()
    }

    fn get_mut_item_registry(&mut self) -> &mut Self::ItemRegistry {
        todo!()
    }

    fn get_mut_chat_type_registry(&mut self) -> &mut Self::ChatTypeRegistry {
        todo!()
    }
}

pub struct AxolotlDataRegistries {
    pub noises: SimpleRegistry<Noise>,
    pub noise_settings: SimpleRegistry<NoiseSetting>,
    pub dimensions: SimpleRegistry<Dimension>,
}
impl DataRegistries for AxolotlDataRegistries {
    type NoiseRegistry = SimpleRegistry<Noise>;
    type NoiseSettingRegistry = SimpleRegistry<NoiseSetting>;
    type DimensionRegistry = SimpleRegistry<Dimension>;

    fn get_noise_registry(&self) -> &Self::NoiseRegistry {
        &self.noises
    }

    fn get_noise_setting_registry(&self) -> &Self::NoiseSettingRegistry {
        &self.noise_settings
    }

    fn get_dimensions_registry(&self) -> &Self::DimensionRegistry {
        &self.dimensions
    }

    fn get_mut_noise_registry(&mut self) -> &mut Self::NoiseRegistry {
        &mut self.noises
    }

    fn get_mut_noise_setting_registry(&mut self) -> &mut Self::NoiseSettingRegistry {
        &mut self.noise_settings
    }

    fn get_mut_dimensions_registry(&mut self) -> &mut Self::DimensionRegistry {
        todo!()
    }
}
