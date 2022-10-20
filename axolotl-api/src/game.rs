use crate::{NamespacedKey, OwnedNameSpaceKey};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::world_gen::noise::density::loading::DensityLoader;
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::{Noise, NoiseSetting};

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

pub trait Game: Sized {
    type World: crate::world::World;
    type Biome: crate::world_gen::biome::Biome;

    type DensityLoader: DensityLoader;
    type Perlin: Perlin<Noise = Noise, Seed = [u8; 16]>;
    type Registries: Registries<Self>;
    type DataRegistries: DataRegistries;
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
registries!(Biome, biome);

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
data_registries!(Noise, noise, NoiseSetting, noise_setting);
pub trait Registry<T> {
    fn register(&mut self, namespace: OwnedNameSpaceKey, item: T);

    fn get(&self, key: &OwnedNameSpaceKey) -> Option<&T>;
}
