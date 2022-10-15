use crate::level::LevelDat;
use axolotl_types::OwnedNameSpaceKey;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BukkitWorldError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub struct BukkitWorld {
    pub world_folder: PathBuf,
    pub level_dat: LevelDat,
    pub dimensions: HashMap<OwnedNameSpaceKey, PathBuf>,
}

impl BukkitWorld {
    pub fn load(world_folder: PathBuf, level_dat: LevelDat) -> Result<Self, BukkitWorldError> {
        Ok(Self {
            world_folder,
            level_dat,
            dimensions: HashMap::new(),
        })
    }
}
