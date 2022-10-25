use crate::region::{ChunkHeader, CompressionType, RegionHeader, RegionLocation};
use crate::Error;
use axolotl_nbt::binary::Binary;
use axolotl_nbt::serde_impl;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::{GzDecoder, ZlibDecoder};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::Serialize;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub trait RegionFileType:
    for<'de> serde::Deserialize<'de> + serde::Serialize + Debug + Clone
{
    fn get_path() -> &'static str
    where
        Self: Sized;

    fn get_xz(&self) -> (i32, i32);
}

#[derive(Debug)]
pub struct RegionFile {
    pub file: PathBuf,
    pub region_header: RegionHeader,
    pub write_buffer: Vec<u8>,
}

impl RegionFile {
    pub fn new(path: PathBuf, initialized: bool) -> Result<Self, Error> {
        if initialized {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)?;
            let region_header = RegionHeader::read_region_header(&mut file)?;
            Ok(Self {
                file: path,
                region_header,
                write_buffer: vec![],
            })
        } else {
            let mut file = OpenOptions::new().read(true).open(&path)?;
            RegionHeader::initialize(&mut file)?;
            let region_header = RegionHeader::default();
            Ok(Self {
                file: path,
                region_header,
                write_buffer: vec![],
            })
        }
    }
    pub fn write_chunk<FileType: RegionFileType + Serialize>(
        &mut self,
        data: FileType,
    ) -> Result<(), Error> {
        let (x, y) = data.get_xz();
        let index = RegionHeader::get_index((x, y)) as usize;
        let location = self.region_header.locations[index];

        // Write the chunk data to the buffer
        let mut writer = ZlibEncoder::new(&mut self.write_buffer, Compression::default());
        serde_impl::to_writer::<Binary, _, _>(&mut writer, &data).map_err(Error::SerdeNBT)?;
        writer.finish()?;
        let length = self.write_buffer.len();

        let sector_count = (length / 4096) + 1;

        if location.0 == 0 {
            self.write_new_chunk(index, length, sector_count)?;
        } else {
            self.rewrite_chunk(index, location, sector_count)?;
        }
        self.write_buffer.clear();
        Ok(())
    }

    fn write_new_chunk(
        &mut self,
        index: usize,
        length: usize,
        sector_count: usize,
    ) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.file)?;
        // Seek to the end of the file
        let i = file.seek(SeekFrom::End(0))?;

        // Write length to the file
        file.write_i32::<BigEndian>(length as i32)?;
        // Write Compression Type to the file
        file.write_u8(2)?;
        // Write the buffer to the file
        file.write_all(&self.write_buffer)?;

        // Pad the file
        for _ in (length + 5)..(4096 * sector_count) {
            file.write_u8(0)?;
        }
        // Flush the file
        file.flush()?;
        // Drop the file
        drop(file);

        // Update the region header
        let location = &mut self.region_header.locations[index as usize];
        location.0 = (i / 4096) as u32;
        location.1 = sector_count as u8;
        Ok(())
    }

    fn rewrite_chunk(
        &mut self,
        index: usize,
        location: RegionLocation,
        sector_count: usize,
    ) -> Result<(), Error> {
        // If old chunk can fit new chunk
        if sector_count <= location.1 as usize {
            let mut file = OpenOptions::new().read(true).write(true).open(&self.file)?;
            file.seek(SeekFrom::Start(location.0 as u64 * 4096))?;
            file.write_i32::<BigEndian>(self.write_buffer.len() as i32)?;
            file.write_u8(2)?;
            file.write_all(&self.write_buffer)?;

            for _ in (self.write_buffer.len() + 5)..(4096 * sector_count) {
                file.write_u8(0)?;
            }
            file.flush()?;
            // Wasted space could occur if the new chunk is smaller than the old chunk
            // TODO: Compact the file
        } else {
            let mut file = OpenOptions::new()
                .read(true)
                .append(true)
                .open(&self.file)?;
            // Seek to the end of the file
            let i = file.seek(SeekFrom::End(0))?;

            // Get Buffer Length
            let number = self.write_buffer.len();
            // Write length to the file
            file.write_i32::<BigEndian>(number as i32)?;
            // Write Compression Type to the file
            file.write_u8(2)?;
            // Write the buffer to the file
            file.write_all(&self.write_buffer)?;

            /// Calculate the sector count
            let sector_count = (number / 4096) + 1;
            /// Pad the file
            for _ in (number + 5)..(4096 * sector_count) {
                file.write_u8(0)?;
            }
            // Flush the file
            file.flush()?;
            // Drop the file
            drop(file);

            // Wasted Space Occurs because the old chunk location is just left there
            // TODO: Compact the file

            // Update the region header
            let location = &mut self.region_header.locations[index as usize];
            location.0 = (i / 4096) as u32;
            location.1 = sector_count as u8;
        }
        Ok(())
    }

    /// A Chunk with 0 length is considered empty
    #[inline]
    pub fn read_chunk_header(&mut self, location: &RegionLocation) -> Result<ChunkHeader, Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file)?;
        if location.0 == 0 {
            return Ok(ChunkHeader::default());
        }
        let calc_offset = location.calc_offset();
        file.seek(SeekFrom::Start(calc_offset as u64))?;

        Self::read_chunk_header_from_file(&mut file)
    }

    fn read_chunk_header_from_file(file: &mut File) -> Result<ChunkHeader, Error> {
        let length = file.read_u32::<BigEndian>()?;
        let compression_type = file.read_u8()?;
        Ok(ChunkHeader {
            length,
            compression_type: compression_type.into(),
        })
    }

    pub fn read_chunk<FileType: RegionFileType>(
        &mut self,
        location: &RegionLocation,
    ) -> Result<Option<(ChunkHeader, FileType)>, Error> {
        if location.0 == 0 {
            return Ok(None);
        }
        let mut file = OpenOptions::new().read(true).open(&self.file)?;
        let calc_offset = location.calc_offset();
        file.seek(SeekFrom::Start(calc_offset as u64))?;

        let header = Self::read_chunk_header_from_file(&mut file)?;
        println!("Reading chunk at {:?}", location);
        println!("Reading chunk at {:?}", header);
        let take = file.take((header.length - 1) as u64);
        #[cfg(feature = "log")]
        log::debug!("Compression Type {:?}", &result.compression_type);
        let value = match &header.compression_type {
            CompressionType::Gzip => {
                serde_impl::from_buf_reader_binary(BufReader::new(GzDecoder::new(take)))?
            }
            CompressionType::Zlib => {
                serde_impl::from_buf_reader_binary(BufReader::new(ZlibDecoder::new(take)))?
            }
            CompressionType::Uncompressed => {
                serde_impl::from_buf_reader_binary(BufReader::new(take))?
            }
            CompressionType::Custom(_) => {
                return Err(Error::InvalidChunkHeader("compression_type"));
            }
        };
        Ok(Some((header, value)))
    }
    pub fn read_chunk_in_place<FileType: RegionFileType>(
        &mut self,
        location: &RegionLocation,
        chunk: &mut FileType,
    ) -> Result<Option<ChunkHeader>, Error> {
        if location.0 == 0 {
            return Ok(None);
        }
        let mut file = OpenOptions::new().read(true).open(&self.file)?;
        let calc_offset = location.calc_offset();
        file.seek(SeekFrom::Start(calc_offset as u64))?;

        let header = Self::read_chunk_header_from_file(&mut file)?;

        let take = file.take((header.length - 1) as u64);
        #[cfg(feature = "log")]
        log::debug!("Compression Type {:?}", &result.compression_type);
        match &header.compression_type {
            CompressionType::Gzip => {
                let mut deserializer = serde_impl::NBTDeserializer::<_, Binary>::new(
                    BufReader::new(GzDecoder::new(take)),
                );
                FileType::deserialize_in_place(&mut deserializer, chunk)?;
            }
            CompressionType::Zlib => {
                let mut deserializer = serde_impl::NBTDeserializer::<_, Binary>::new(
                    BufReader::new(ZlibDecoder::new(take)),
                );
                FileType::deserialize_in_place(&mut deserializer, chunk)?;
            }
            CompressionType::Uncompressed => {
                let mut deserializer =
                    serde_impl::NBTDeserializer::<_, Binary>::new(BufReader::new(take));
                FileType::deserialize_in_place(&mut deserializer, chunk)?;
            }
            CompressionType::Custom(_) => {
                return Err(Error::InvalidChunkHeader("compression_type"));
            }
        };
        Ok(Some(header))
    }

    /// Saves the RegionHeader to the file
    pub fn save(&mut self) -> Result<(), Error> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.file)?;
        file.seek(SeekFrom::Start(0))?;
        self.region_header.write_region(&mut file)?;
        Ok(())
    }
}
