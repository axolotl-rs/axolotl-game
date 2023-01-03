use std::collections::HashMap;
use std::fmt::Debug;

use paste::paste;
use serde::{Deserialize, Serialize};

use crate::chat::ChatType;
use crate::item::block::Block;
use crate::item::{Item, ItemStack};
use crate::world::World;
use crate::world_gen::biome::Biome;
use crate::world_gen::dimension::Dimension;
use crate::world_gen::noise::density::loading::DensityLoader;
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::{Noise, NoiseSetting};
use crate::OwnedNameSpaceKey;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SupportedVersion {
    pub protocol_version: i64,
    pub world_version: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AxolotlVersion {
    pub game_version: String,
    pub axolotl_version: String,
    pub protocol_version: i64,
    pub supported_versions: HashMap<String, SupportedVersion>,
}
///
pub trait Game: Sized + Debug {
    type Biome: Biome;
    type World: World;
    type Block: Block<Self>;
    type Item: Item<Self>;
    type ItemStack: ItemStack<Self>;
    type DensityLoader: DensityLoader;
    type Registries: Registries<Self>;
    type DataRegistries: DataRegistries;
    type ChatType: ChatType;

    fn registries(&self) -> &Self::Registries;
    fn mut_registries(&mut self) -> &mut Self::Registries;

    fn data_registries(&self) -> &Self::DataRegistries;
    fn mut_data_registries(&mut self) -> &mut Self::DataRegistries;
}

macro_rules! registries {
    ($($t:expr, $name:ident),*) => {
        pub trait Registries<G: Game>{
            paste!{
               $(
                type [<$t Registry>]: Registry<G::$t>;
                )*
                $(
                    fn [<get_$name _registry>](&self) -> &Self::[<$t Registry>];
                )*
                $(
                    fn [<get_mut_$name _registry>](&mut self) -> &mut Self::[<$t Registry>];
                )*
            }
        }
    };
}
registries!(Biome, biome, Block, block, Item, item, ChatType, chat_type);

macro_rules! data_registries {
    ($($t:ty, $name:ident),*) => {
        /// Data Registries are registries that store pure data no logic
        pub trait DataRegistries{
            paste!{
               $(
                type [<$t Registry>]: Registry<$t>;
                )*
                $(
                    fn [<get_$name _registry>](&self) -> &Self::[<$t Registry>];
                )*
                $(
                    fn [<get_mut_$name _registry>](&mut self) -> &mut Self::[<$t Registry>];
                )*
            }
        }
    };
}
data_registries!(
    Noise,
    noise,
    NoiseSetting,
    noise_setting,
    Dimension,
    dimensions
);

pub trait Registry<T> {
    fn register(&mut self, namespace: impl Into<String>, item: T) -> usize;
    fn register_with_id(&mut self, namespace: impl Into<String>, id: usize, item: T);

    fn get_by_id(&self, id: usize) -> Option<&T>;

    fn get_id(&self, namespace: impl AsRef<str>) -> Option<usize>;

    fn get_by_namespace(&self, namespace: impl AsRef<str>) -> Option<&T>;
    fn get_by_namespace_key(&self, key: &OwnedNameSpaceKey) -> Option<&T> {
        self.get_by_namespace(key.to_string())
    }
}
