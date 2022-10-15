use crate::level::LevelDat;

use axolotl_nbt::serde_impl;
use axolotl_types::OwnedNameSpaceKey;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VanillaWorldError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SerdeNBT(#[from] serde_impl::Error),
}

pub struct VanillaWorld {
    pub world_folder: PathBuf,
    pub level_dat: LevelDat,
    pub dimensions: HashMap<OwnedNameSpaceKey, PathBuf>,
}
