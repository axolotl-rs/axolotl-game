pub mod player;

use crate::world::level::accessor::{IntoRawChunk, LevelReader, LevelWriter, RawChunk};
use crate::{AxolotlGame, Error};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_world::entity::RawEntities;
use axolotl_world::region::file::{RegionFile, RegionFileType};
use axolotl_world::region::RegionHeader;
use axolotl_world::world::axolotl::AxolotlWorld as RawWorld;
use itoa::Buffer;
use log::{debug, warn};
use parking_lot::lock_api::RawMutex;
use parking_lot::Mutex;
use std::fs::OpenOptions;
use std::sync::atomic::AtomicU8;

use tux_lockfree::map::Removed;
use tux_lockfree::prelude::{Map, Queue};

pub const MAX_NUMBER_OPEN_REGIONS: u8 = 16;
#[derive(Debug)]
pub struct ActiveRegion {
    pub chunks: RegionFile,
    pub entities: RegionFile,
}
#[derive(Debug)]
pub struct Minecraft19WorldAccessor<'game> {
    pub open_regions: AtomicU8,
    pub active_regions: Map<(i32, i32), Mutex<ActiveRegion>>,
    pub world: RawWorld,
    pub dead_chunks: Queue<RawChunk>,
    pub dead_regions: Queue<(RegionHeader, Vec<u8>)>,
    pub game: &'game AxolotlGame,
}
impl<'game> Minecraft19WorldAccessor<'game> {
    pub fn clean(&self) {
        for x in self.dead_regions.pop_iter() {
            debug!("Removing dead region {:?}", x);
        }
        for x in self.dead_chunks.pop_iter() {
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
            if let Some((mut header, buffer)) = self.dead_regions.pop() {
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
            let mut file = OpenOptions::new()
                .read(true)
                .create(true)
                .write(true)
                .open(&buf)?;
            if let Some((mut header, buffer)) = self.dead_regions.pop() {
                header.initialize_and_zero(&mut file)?;
                let region = RegionFile {
                    file: buf,
                    region_header: header,
                    write_buffer: buffer,
                };
                Ok(region)
            } else {
                let region = RegionFile::new(buf, false)?;
                Ok(region)
            }
        };
    }
    pub fn close_region(&self, region_loc: (i32, i32)) {
        if let Some(region) = self.active_regions.remove(&region_loc) {
            match Removed::try_into(region) {
                Ok((loc, v)) => {
                    let mut region = v.into_inner();
                    if let Err(e) = region.entities.save() {
                        warn!("Failed to save entities for region {:?}: {}", loc, e);
                    }
                    if let Err(e) = region.chunks.save() {
                        warn!("Failed to save chunks for region {:?}: {}", loc, e);
                    }
                    self.dead_regions
                        .push((region.chunks.region_header, region.chunks.write_buffer));
                    self.dead_regions
                        .push((region.entities.region_header, region.entities.write_buffer));
                    debug!("Closed region {:?}", loc);
                }
                Err(err) => {
                    warn!("Failed to remove region from active regions.");
                    self.active_regions.reinsert(err);
                }
            };
        }
    }
    pub fn region<After, R>(&self, pos: &ChunkPos, after: After) -> Result<R, Error>
    where
        After: FnOnce(&mut ActiveRegion) -> Result<R, Error>,
    {
        let region_loc = (pos.0 / 32, pos.1 / 32);
        if let Some(region) = self.active_regions.get(&region_loc) {
            let mut guard = region.val().lock();
            after(&mut guard)
        } else {
            if self.open_regions.load(std::sync::atomic::Ordering::Relaxed)
                > MAX_NUMBER_OPEN_REGIONS
            {
                let mut vec = Vec::with_capacity((MAX_NUMBER_OPEN_REGIONS / 8) as usize);

                for read in self.active_regions.iter() {
                    if !read.val().is_locked() {
                        vec.push(*read.key());
                    }
                }
                for key in vec.into_iter() {
                    self.close_region(key);
                }
            }
            let mut active_region = ActiveRegion {
                chunks: self.open_region_file::<RawChunk>(region_loc)?,
                entities: self.open_region_file::<RawEntities>(region_loc)?,
            };
            let result = after(&mut active_region);
            let guard = Mutex::new(active_region);
            if let Some(v) = self.active_regions.insert(region_loc, guard) {
                warn!("Region loaded twice: {:?}", v);
            }
            result
        }
    }
}
impl<'game> LevelReader<'game> for Minecraft19WorldAccessor<'game> {
    type Error = crate::Error;

    fn get_chunk_into(
        &self,
        chunk_pos: &ChunkPos,
        chunk: &mut impl IntoRawChunk<'game>,
    ) -> Result<bool, Self::Error> {
        self.region(chunk_pos, |region| {
            let index = RegionHeader::get_index(chunk_pos) as usize;
            if let Some(region_loc) = region.chunks.region_header.locations.get(index) {
                let region_loc = *region_loc;
                if let Some(mut v) = self.dead_chunks.pop() {
                    if region
                        .chunks
                        .read_chunk_in_place(&region_loc, &mut v)?
                        .is_some()
                    {
                        chunk.load_from_chunk(self.game, &mut v, None);

                        self.dead_chunks.push(v);
                        Ok(true)
                    } else {
                        self.dead_chunks.push(v);
                        return Ok(false);
                    }
                } else {
                    if let Some((_, mut raw_chunk)) = region.chunks.read_chunk(&region_loc)? {
                        chunk.load_from_chunk(self.game, &mut raw_chunk, None);
                        self.dead_chunks.push(raw_chunk);
                        Ok(true)
                    } else {
                        return Ok(false);
                    }
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
                if let Some(mut v) = self.dead_chunks.pop() {
                    if region
                        .chunks
                        .read_chunk_in_place(&region_loc, &mut v)?
                        .is_some()
                    {
                        Ok(Some(v))
                    } else {
                        return Ok(None);
                    }
                } else {
                    if let Some((_, raw_chunk)) = region.chunks.read_chunk(&region_loc)? {
                        Ok(Some(raw_chunk))
                    } else {
                        return Ok(None);
                    }
                }
            } else {
                warn!("Chunk Outside Bounds: {:?}", chunk_pos);
                Ok(None)
            }
        })
    }
}
impl<'game> LevelWriter<'game> for Minecraft19WorldAccessor<'game> {
    type Error = crate::Error;

    fn save_chunk(
        &self,
        chunk_pos: ChunkPos,
        chunk: impl IntoRawChunk<'game>,
    ) -> Result<(), Self::Error> {
        self.region(&chunk_pos, |region| {
            let index = RegionHeader::get_index(&chunk_pos) as usize;
            if let Some(region_loc) = region.chunks.region_header.locations.get(index) {
                let _region_loc = *region_loc;
                if let Some(mut v) = self.dead_chunks.pop() {
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
