pub mod level_dat;

use crate::entity::player::PlayerData;

use crate::world::axolotl::level_dat::AxolotlLevelDat;
use crate::world::World;
use axolotl_nbt::serde_impl;
use axolotl_types::OwnedNameSpaceKey;

use std::collections::HashMap;

use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AxolotlWorldError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SerdeNBT(#[from] serde_impl::Error),
    #[error("Missing Axolotl Required Param: {0}")]
    MissingAxolotlParam(&'static str),
}
#[derive(Debug, Clone)]
pub struct AxolotlWorld {
    pub world_folder: PathBuf,
    pub player_folder: PathBuf,
    pub level_dat: AxolotlLevelDat,
    /// Inside Axolotl these worlds are the connected ones via portals
    pub dimensions: HashMap<OwnedNameSpaceKey, PathBuf>,
}

impl World for AxolotlWorld {
    type Error = AxolotlWorldError;
    type LevelDat = AxolotlLevelDat;

    fn load(world_folder: PathBuf, level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let player_folder = world_folder
            .join(level_dat.axolotl_player_data.as_str())
            .canonicalize()?;
        let dimensions = HashMap::new();

        Ok(Self {
            world_folder,
            player_folder,
            level_dat,
            dimensions,
        })
    }

    fn create(_world_folder: PathBuf, _level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_dimensions(&self) -> &HashMap<OwnedNameSpaceKey, PathBuf> {
        &self.dimensions
    }

    fn get_player_file(&self, uuid: impl Into<Uuid>) -> Result<PlayerData, Self::Error> {
        let player_file = self
            .player_folder
            .join(format!("{}.dat", uuid.into().hyphenated().to_string()));
        let file = std::fs::File::open(player_file)?;
        let reader = std::io::BufReader::new(file);

        let player_data: PlayerData = serde_impl::from_buf_reader_binary(reader)?;
        Ok(player_data)
    }

    fn get_level_dat(&self) -> &Self::LevelDat {
        todo!()
    }

    fn get_level_dat_mut(&mut self) -> &mut Self::LevelDat {
        todo!()
    }

    fn get_world_folder(&self) -> &PathBuf {
        todo!()
    }
}
