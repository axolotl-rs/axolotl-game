use crate::entity::player::PlayerData;

use crate::world::World;
use axolotl_nbt::serde_impl;
use axolotl_types::OwnedNameSpaceKey;

use std::collections::HashMap;

use crate::level::{LevelDat, RootWrapper};
use axolotl_nbt::value::NameLessValue;
use flate2::write::GzEncoder;
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
    pub level_dat: LevelDat,
    /// Inside Axolotl these worlds are the connected ones via portals
    pub dimensions: HashMap<OwnedNameSpaceKey, PathBuf>,
}
impl AxolotlWorld {
    pub fn get_player_dat(level: &LevelDat) -> PathBuf {
        level
            .other
            .get("player_dat")
            .map(|s| {
                if let NameLessValue::String(s) = s {
                    s.as_str()
                } else {
                    "playerdata"
                }
            })
            .unwrap_or("playerdata")
            .into()
    }
}
impl World for AxolotlWorld {
    type Error = AxolotlWorldError;
    type LevelDat = LevelDat;

    fn load(world_folder: PathBuf, level_dat: Self::LevelDat) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let player_folder = world_folder
            .join(Self::get_player_dat(&level_dat))
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
        let mut level_dat_file = GzEncoder::new(
            std::fs::File::create(level_dat_path)?,
            flate2::Compression::default(),
        );
        let wrap = RootWrapper { data: level_dat };
        serde_impl::to_writer(&mut level_dat_file, &wrap)?;
        let level_dat = wrap.data;
        level_dat_file.finish()?;
        let player_folder = world_folder.join(Self::get_player_dat(&level_dat));
        if !player_folder.exists() {
            std::fs::create_dir_all(&player_folder)?;
        }
        let entities_folder = world_folder.join("entities");
        if !entities_folder.exists() {
            std::fs::create_dir_all(&entities_folder)?;
        }
        let regions = world_folder.join("region");

        if !regions.exists() {
            std::fs::create_dir_all(&regions)?;
        }

        Self::load(world_folder, level_dat)
    }

    fn get_dimensions(&self) -> &HashMap<OwnedNameSpaceKey, PathBuf> {
        &self.dimensions
    }

    fn get_player_file(&self, uuid: impl Into<Uuid>) -> Result<PlayerData, Self::Error> {
        let player_file = self
            .player_folder
            .join(format!("{}.dat", uuid.into().hyphenated()));
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
