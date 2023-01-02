pub mod player;

use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter, RawChunk};
use crate::Error::WorldError;
use crate::{AxolotlGame, Error};
use ahash::AHashMap;
use axolotl_api::world::World;
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_nbt::serde_impl;
use axolotl_world::entity::RawEntities;
use axolotl_world::level;
use axolotl_world::level::{DataPacks, LevelDat, MinecraftVersion, RootWrapper, WorldGenSettings};
use axolotl_world::region::file::{RegionFile, RegionFileType};
use axolotl_world::region::RegionHeader;
use axolotl_world::world::axolotl::{AxolotlWorld as RawWorld, AxolotlWorldError};
use axolotl_world::world::World as RawWorldTrait;
use flate2::read::GzDecoder;
use itoa::Buffer;
use log::{debug, info, warn};
use parking_lot::lock_api::{RawMutex, RwLockWriteGuard};
use parking_lot::{Mutex, RawRwLock, RwLock};
use serde_json::Value;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;

pub const MAX_NUMBER_OPEN_REGIONS: usize = 16;
#[derive(Debug)]
pub struct ActiveRegion {
    pub chunks: RegionFile,
    pub entities: RegionFile,
}
type RegionRef = Arc<Mutex<ActiveRegion>>;
#[derive(Debug)]
pub struct Minecraft19WorldAccessor<W: World> {
    pub active_regions: RwLock<AHashMap<(i32, i32), RegionRef>>,
    pub world: RawWorld,
    pub dead_chunks: Mutex<VecDeque<RawChunk>>,
    pub dead_regions: Mutex<VecDeque<(RegionHeader, Vec<u8>)>>,
    pub game: Arc<AxolotlGame<W>>,
}

impl<W: World> Minecraft19WorldAccessor<W> {
    fn new(game: Arc<AxolotlGame<W>>, world: RawWorld) -> Self {
        Self {
            active_regions: RwLock::new(AHashMap::with_capacity(MAX_NUMBER_OPEN_REGIONS)),
            world,
            dead_chunks: Mutex::new(VecDeque::with_capacity(8)),
            dead_regions: Mutex::new(VecDeque::with_capacity(8)),
            game,
        }
    }
    pub fn load(game: Arc<AxolotlGame<W>>, path: PathBuf) -> Result<Self, Error> {
        let level_dat_file = path.join("level.dat");
        if !level_dat_file.exists() {
            return Err(Error::WorldError(axolotl_world::Error::WorldDoesNotExist));
        }
        let mut file =
            std::fs::File::open(level_dat_file).map(|r| BufReader::new(GzDecoder::new(r)))?;
        let level_dat: RootWrapper = serde_impl::from_buf_reader_binary(file)?;
        let world = RawWorld::load(path, level_dat.data)?;
        Ok(Self::new(game, world))
    }
    pub fn create(
        game: Arc<AxolotlGame<W>>,
        world_gen: impl Into<WorldGenSettings>,
        path: PathBuf,
        name: String,
    ) -> Result<Self, Error> {
        let world = RawWorld::create(
            path,
            LevelDat {
                version: MinecraftVersion {
                    name: "1.19.2".to_string(),
                    id: 3120,
                    snapshot: false,
                    series: "main".to_string(),
                },
                data_packs: DataPacks {
                    disabled: vec![],
                    enabled: vec!["vanilla".to_string()],
                },
                game_rules: level::default_game_rules(),
                world_gen_settings: world_gen.into(),
                initialized: true,
                was_modded: true,
                server_brands: vec!["Axolotl".to_string()],
                level_name: name,
                version_num: 19133,
                data_version: 3120,
                ..Default::default()
            },
        )?;
        Ok(Self::new(game, world))
    }
    pub fn clean(&self) {
        let mut guard = self.dead_regions.lock();
        while let Some(x) = guard.pop_front() {
            debug!("Removing dead region {:?}", x);
        }
        drop(guard);
        let mut guard = self.dead_chunks.lock();
        while let Some(x) = guard.pop_front() {
            debug!("Removing dead chunk {:?}", x);
        }
    }
    pub fn open_region_file<RegionType: RegionFileType>(
        &self,
        chunk_pos: (i32, i32),
    ) -> Result<RegionFile, Error> {
        let buf = self
            .world
            .world_folder
            .join(RegionType::get_path())
            .join(format!(
                "r.{}.{}.mca",
                Buffer::new().format(chunk_pos.0),
                Buffer::new().format(chunk_pos.1)
            ));
        return if buf.exists() {
            let mut file = OpenOptions::new().read(true).write(true).open(&buf)?;
            if let Some((mut header, buffer)) = self.dead_regions.lock().pop_front() {
                RegionHeader::replace_region_header(
                    &mut file,
                    &mut header.locations,
                    &mut header.timestamps,
                )?;
                let region = RegionFile {
                    file: buf,
                    region_header: header,
                    write_buffer: buffer,
                };
                Ok(region)
            } else {
                let region = RegionFile::new(buf, true)?;
                Ok(region)
            }
        } else {
            debug!("Creating new region file {:?}", buf);
            let mut file = OpenOptions::new()
                .read(true)
                .create_new(true)
                .write(true)
                .open(&buf)?;
            if let Some((mut header, buffer)) = self.dead_regions.lock().pop_front() {
                header.initialize_and_zero(&mut file)?;
                let region = RegionFile {
                    file: buf,
                    region_header: header,
                    write_buffer: buffer,
                };
                Ok(region)
            } else {
                info!("Initializing new region file {:?}", buf);
                let mut region = RegionFile::new(buf, false)?;
                Ok(region)
            }
        };
    }
    fn close_inner(&self, region_loc: &(i32, i32), mut region: ActiveRegion) {
        if let Err(e) = region.entities.save() {
            warn!("Failed to save entities for region {:?}: {}", region_loc, e);
        }
        if let Err(e) = region.chunks.save() {
            warn!("Failed to save chunks for region {:?}: {}", region_loc, e);
        }
        self.dead_regions
            .lock()
            .push_back((region.chunks.region_header, region.chunks.write_buffer));
        self.dead_regions
            .lock()
            .push_back((region.entities.region_header, region.entities.write_buffer));
        debug!("Closed region {:?}", region_loc);
    }
    pub fn close_region(&self, region_loc: (i32, i32)) {
        if let Some(region) = self.active_regions.write().remove(&region_loc) {
            let mut region = match Arc::try_unwrap(region) {
                Ok(ok) => ok,
                Err(err) => {
                    warn!(
                        "Attempted to close region {:?} but it was still in use",
                        region_loc
                    );
                    self.active_regions.write().insert(region_loc, err);

                    return;
                }
            };
            let region = region.into_inner();
            self.close_inner(&region_loc, region);
        }
    }
    pub fn region<After, R>(&self, pos: &ChunkPos, after: After) -> Result<R, Error>
    where
        After: FnOnce(&mut ActiveRegion) -> Result<R, Error>,
    {
        let region_loc = (pos.0 / 32, pos.1 / 32);
        let guard = self.active_regions.read();
        if let Some(region) = guard.get(&region_loc).cloned() {
            drop(guard);
            let mut guard = region.lock();
            after(&mut guard)
        } else {
            drop(guard);
            let mut guard = self.active_regions.write();
            if guard.len() >= MAX_NUMBER_OPEN_REGIONS {
                self.attempt_region_clean(&mut guard)
            }
            debug!("Opening region {:?}", region_loc);
            let mut active_region = ActiveRegion {
                chunks: self.open_region_file::<RawChunk>(region_loc)?,
                entities: self.open_region_file::<RawEntities>(region_loc)?,
            };
            let result = after(&mut active_region);
            let value = Arc::new(Mutex::new(active_region));

            if let Some(v) = guard.insert(region_loc, value) {
                warn!("Region loaded twice: {:?}", v);
            }
            result
        }
    }

    /// Attempts to clean up regions by closing ones without active references
    fn attempt_region_clean(
        &self,
        guard: &mut RwLockWriteGuard<RawRwLock, AHashMap<(i32, i32), RegionRef>>,
    ) {
        let len = MAX_NUMBER_OPEN_REGIONS / 2;
        let mut vec = Vec::with_capacity(len);

        for (index, (key, val)) in guard.iter().enumerate() {
            if Arc::strong_count(val) == 1 && !val.is_locked() {
                vec.push(*key);
            }
            if index > len {
                break;
            }
        }
        if vec.is_empty() {
            warn!("No regions to close increasing number of open regions");
        } else {
            for (loc, region) in vec
                .into_iter()
                .map(|x| (x, Arc::try_unwrap(guard.remove(&x).unwrap()).unwrap()))
                .map(|(loc, value)| (loc, value.into_inner()))
            {
                self.close_inner(&loc, region);
            }
        }
    }
    pub fn force_close_all(&self) {
        let mut guard = self.active_regions.write();
        for (loc, region) in guard
            .drain()
            .map(|(loc, value)| (loc, Arc::try_unwrap(value).unwrap()))
            .map(|(loc, value)| (loc, value.into_inner()))
        {
            self.close_inner(&loc, region);
        }
    }
}
impl<W: World> LevelReader<W> for Minecraft19WorldAccessor<W> {
    type Error = crate::Error;

    fn get_chunk_into(
        &self,
        chunk_pos: &ChunkPos,
        chunk: &mut impl IntoRawChunk<W>,
    ) -> Result<bool, Self::Error> {
        self.region(chunk_pos, |region| {
            let index = RegionHeader::get_index(chunk_pos) as usize;
            if let Some(region_loc) = region.chunks.region_header.locations.get(index) {
                let region_loc = *region_loc;
                if let Some(mut v) = self.dead_chunks.lock().pop_front() {
                    if region
                        .chunks
                        .read_chunk_in_place(&region_loc, &mut v)?
                        .is_some()
                    {
                        chunk.load_from_chunk(self.game.clone(), &mut v, None);

                        self.dead_chunks.lock().push_back(v);
                        Ok(true)
                    } else {
                        self.dead_chunks.lock().push_back(v);
                        Ok(false)
                    }
                } else if let Some((_, mut raw_chunk)) = region.chunks.read_chunk(&region_loc)? {
                    chunk.load_from_chunk(self.game.clone(), &mut raw_chunk, None);
                    self.dead_chunks.lock().push_back(raw_chunk);
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                warn!("Chunk Outside Bounds: {:?}", chunk_pos);
                Ok(false)
            }
        })
    }

    fn get_chunk(&self, chunk_pos: &ChunkPos) -> Result<Option<RawChunk>, Self::Error> {
        self.region(chunk_pos, |region| {
            let index = RegionHeader::get_index(chunk_pos) as usize;
            if let Some(region_loc) = region.chunks.region_header.locations.get(index) {
                let region_loc = *region_loc;
                if let Some(mut v) = self.dead_chunks.lock().pop_front() {
                    if region
                        .chunks
                        .read_chunk_in_place(&region_loc, &mut v)?
                        .is_some()
                    {
                        Ok(Some(v))
                    } else {
                        Ok(None)
                    }
                } else if let Some((_, raw_chunk)) = region.chunks.read_chunk(&region_loc)? {
                    Ok(Some(raw_chunk))
                } else {
                    Ok(None)
                }
            } else {
                warn!("Chunk Outside Bounds: {:?}", chunk_pos);
                Ok(None)
            }
        })
    }
}
impl<W: World> LevelWriter<W> for Minecraft19WorldAccessor<W> {
    type Error = crate::Error;

    fn save_chunk(
        &self,
        chunk_pos: ChunkPos,
        chunk: impl IntoRawChunk<W>,
    ) -> Result<(), Self::Error> {
        self.region(&chunk_pos, |region| {
            let index = RegionHeader::get_index(chunk_pos) as usize;
            if let Some(region_loc) = region.chunks.region_header.locations.get(index) {
                let _region_loc = *region_loc;
                if let Some(mut v) = self.dead_chunks.lock().pop_front() {
                    chunk.into_raw_chunk_use(&mut v);
                    region.chunks.write_chunk(v)?;
                    Ok(())
                } else {
                    let raw_chunk = chunk.into_raw_chunk();
                    region.chunks.write_chunk(raw_chunk)?;
                    Ok(())
                }
            } else {
                warn!("Chunk Outside Bounds: {:?}", chunk_pos);
                Ok(())
            }
        })
    }

    fn save_chunks(
        &self,
        _chunks: impl Iterator<Item = (ChunkPos, RawChunk)>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
