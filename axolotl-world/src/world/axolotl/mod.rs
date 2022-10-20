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

    fn create(world_folder: PathBuf, level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        if !world_folder.exists() {
            std::fs::create_dir_all(&world_folder)?;
        }
        let level_dat_path = world_folder.join("level.dat");
        if level_dat_path.exists() {
            return Err(AxolotlWorldError::IO(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "level.dat already exists",
            )));
        }
        let mut level_dat_file = std::fs::File::create(level_dat_path)?;
        serde_impl::to_writer(&mut level_dat_file, &level_dat)?;
        let player_folder = world_folder
            .join(level_dat.axolotl_player_data.as_str())
            .canonicalize()?;
        if !player_folder.exists() {
            std::fs::create_dir_all(&player_folder)?;
        }
        let entities_folder = player_folder.join("entities");
        if !entities_folder.exists() {
            std::fs::create_dir_all(&entities_folder)?;
        }
        let regions = player_folder.join("regions");

        if !regions.exists() {
            std::fs::create_dir_all(&regions)?;
        }

        Ok(Self::load(world_folder, level_dat)?)
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
