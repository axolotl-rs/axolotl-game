pub mod axolotl;
pub mod bukkit;
pub mod vanilla;

use crate::entity::player::PlayerData;

use axolotl_types::OwnedNameSpaceKey;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use std::path::PathBuf;
use uuid::Uuid;

pub trait World {
    type Error: Error;
    type LevelDat: Serialize + DeserializeOwned + Debug;

    fn load(world_folder: PathBuf, level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Will create a new world
    fn create(world_folder: PathBuf, level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn get_dimensions(&self) -> &HashMap<OwnedNameSpaceKey, PathBuf>;

    fn get_player_file(&self, uuid: impl Into<Uuid>) -> Result<PlayerData, Self::Error>;

    fn get_level_dat(&self) -> &Self::LevelDat;

    fn get_level_dat_mut(&mut self) -> &mut Self::LevelDat;

    fn get_world_folder(&self) -> &PathBuf;
}
