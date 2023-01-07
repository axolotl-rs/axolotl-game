#![allow(unused, clippy::from_over_into)]

extern crate core;

use std::any::type_name;
use std::collections::VecDeque;
use std::fmt::{Debug, DebugStruct, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::path::{Path, PathBuf};

use axolotl_nbt::serde_impl;
pub use flume::{bounded, unbounded, Receiver, Sender};
//pub use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use flume::Drain;
use log::{debug, info};
use thiserror::Error;

use axolotl_api::game::{AxolotlVersion, DataRegistries, Game, Registries, Registry};
use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::biome::vanilla::DataPackBiome;
use axolotl_api::world_gen::dimension::Dimension;
use axolotl_api::world_gen::noise::{Noise, NoiseSetting};
use axolotl_api::{NamespacedId, NamespacedKey};
use axolotl_items::blocks::MinecraftBlock;
use axolotl_items::items::MinecraftItem;
use axolotl_world::level::MinecraftVersion;
use registry::SimpleRegistry;

use crate::chat::AxolotlChatType;
use crate::item_stack::AxolotlItemStack;
use crate::world::generator::AxolotlDensityLoader;
use crate::world::perlin::GameNoise;

pub mod chat;
pub mod item_stack;
pub mod registry;
pub mod world;

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
    AxolotlWorldError(#[from] axolotl_world::world::axolotl::AxolotlWorldError),
    #[error(transparent)]
    NbtError(#[from] axolotl_nbt::NBTError),
    #[error(transparent)]
    SerdeError(#[from] serde_impl::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

pub(crate) use get_type;

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
pub struct AxolotlGame<W: World> {
    pub data_registries: AxolotlDataRegistries,
    pub registries: AxolotlRegistries<W>,
    pub density_loader: AxolotlDensityLoader,
    pub minecraft_version: MinecraftVersion,
    pub axolotl_version: AxolotlVersion,
}
impl<W: World> AxolotlGame<W> {
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
        let data_registries = AxolotlDataRegistries::new(&config.data_dump)?;
        let density_loader = AxolotlDensityLoader(SimpleRegistry::load_from_path(
            config
                .data_dump
                .join("data")
                .join("minecraft")
                .join("worldgen")
                .join("density_function"),
        )?);

        // TODO: Load Data Packs
        Ok(Self {
            data_registries,
            registries: AxolotlRegistries::new(&config.axolotl_data, &config.data_dump)?,
            density_loader,
            minecraft_version,
            axolotl_version,
        })
    }
    pub fn get_block(&self, key: impl NamespacedKey) -> Option<&MinecraftBlock<Self>> {
        self.registries.blocks.get_by_namespace(format!(
            "{}:{}",
            key.get_namespace(),
            key.get_key()
        ))
    }
}
impl<W: World> Debug for AxolotlGame<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AxolotlGame")
            .field("axolotl_version", &self.axolotl_version.axolotl_version)
            .field("minecraft_version", &self.minecraft_version.series)
            .field("AxolotlRegistries", &self.registries)
            .field("AxolotlDataRegistries", &self.data_registries)
            .finish()
    }
}
impl<W: World> Game for AxolotlGame<W> {
    type Biome = DataPackBiome;
    type World = W;
    type Block = MinecraftBlock<Self>;
    type Item = MinecraftItem<Self>;
    type ItemStack = AxolotlItemStack<W>;

    type DensityLoader = AxolotlDensityLoader;
    type Registries = AxolotlRegistries<W>;
    type DataRegistries = AxolotlDataRegistries;
    type ChatType = AxolotlChatType;

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
pub struct AxolotlRegistries<W: World> {
    pub biomes: SimpleRegistry<DataPackBiome>,
    pub blocks: SimpleRegistry<MinecraftBlock<AxolotlGame<W>>>,
    pub chat_types: SimpleRegistry<AxolotlChatType>,
}
impl<W: World> Debug for AxolotlRegistries<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<W>();
        f.debug_struct("AxolotlRegistries")
            .field("biomes", &self.biomes.values.len())
            .field("blocks", &self.blocks.values.len())
            .field("chat_types", &self.chat_types.values.len())
            .field("World Type", &name)
            .finish()
    }
}
impl<W: World> AxolotlRegistries<W> {
    fn new(axolotl_data: impl AsRef<Path>, data_dump: impl AsRef<Path>) -> Result<Self, Error> {
        let chat_types = SimpleRegistry::load_from_path(
            data_dump
                .as_ref()
                .join("data")
                .join("minecraft")
                .join("chat_type"),
        )?;
        let materials = axolotl_items::load_materials(axolotl_data.as_ref().to_path_buf()).unwrap();
        let mut block_registry = SimpleRegistry::new();
        axolotl_items::load_blocks(
            axolotl_data.as_ref().to_path_buf(),
            data_dump.as_ref().to_path_buf(),
            &materials,
            &mut block_registry,
        )
        .unwrap();

        Ok(AxolotlRegistries {
            biomes: SimpleRegistry::load_from_path(
                data_dump
                    .as_ref()
                    .join("data")
                    .join("minecraft")
                    .join("worldgen")
                    .join("biome"),
            )?,
            blocks: block_registry,
            chat_types,
        })
    }
}
impl<W: World> Registries<AxolotlGame<W>> for AxolotlRegistries<W> {
    type BiomeRegistry = SimpleRegistry<DataPackBiome>;
    type BlockRegistry = SimpleRegistry<MinecraftBlock<AxolotlGame<W>>>;
    type ItemRegistry = SimpleRegistry<MinecraftItem<AxolotlGame<W>>>;
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
impl Debug for AxolotlDataRegistries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AxolotlDataRegistries")
            .field("noises", &self.noises.values.len())
            .field("noise_settings", &self.noise_settings.values.len())
            .field("dimensions", &self.dimensions.values.len())
            .finish()
    }
}
impl AxolotlDataRegistries {
    pub fn new(data_dump: impl AsRef<Path>) -> Result<Self, Error> {
        let data_dump = data_dump.as_ref();
        let noises = SimpleRegistry::load_from_path(
            data_dump
                .join("data")
                .join("minecraft")
                .join("worldgen")
                .join("noise"),
        )?;
        let noise_settings = SimpleRegistry::load_from_path(
            data_dump
                .join("data")
                .join("minecraft")
                .join("worldgen")
                .join("noise_settings"),
        )?;
        let dimensions = SimpleRegistry::load_from_path(
            data_dump
                .join("data")
                .join("minecraft")
                .join("dimension_type"),
        )?;
        Ok(Self {
            noises,
            noise_settings,
            dimensions,
        })
    }
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
