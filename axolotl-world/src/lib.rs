use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use axolotl_nbt::{serde_impl, NBTError};
use flate2::read::GzDecoder;
use thiserror::Error;

use crate::level::LevelDat;

pub mod chunk;
pub mod entity;
pub mod item;
pub mod level;
pub mod region;
pub mod world;
#[test]
pub fn test_build() {
    println!("Intellij Rust is weird. This makes it happy.");
}
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nbt(#[from] NBTError),
    #[error(transparent)]
    SerdeNBT(#[from] serde_impl::Error),
    #[error("Invalid chunk header: {0}")]
    InvalidChunkHeader(&'static str),
    #[error("World does not exist")]
    WorldDoesNotExist,
}

impl Error {
    pub fn missing_parent_dir(path: &Path) -> Self {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Parent directory of {} does not exist", path.display()),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldStyle {
    Vanilla,
    Bukkit,
    Axolotl,
}

///
/// Returns the World Style and the LevelDat
///
/// # Errors
/// Will Error out if level.dat is not found
///
pub fn get_world_type_and_level_dat(
    path: impl AsRef<Path>,
) -> Result<(WorldStyle, LevelDat), Error> {
    let directory = path.as_ref();
    let mut level_dat = directory.join("level.dat");
    if !level_dat.exists() {
        level_dat = directory.join("world").join("level.dat");
        if !level_dat.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Could not find level.dat in {} or {}/world",
                    directory.display(),
                    directory.display()
                ),
            )));
        }
    }
    let level_dat: LevelDat =
        serde_impl::from_buf_reader_binary(BufReader::new(GzDecoder::new(File::open(level_dat)?)))?;
    let style = if level_dat.other.contains_key("Bukkit.Version") {
        WorldStyle::Bukkit
    } else if level_dat.other.contains_key("Axolotl.Version") {
        WorldStyle::Axolotl
    } else {
        // TODO check for other world layouts
        WorldStyle::Vanilla
    };
    Ok((style, level_dat))
}
