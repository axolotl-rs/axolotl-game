use crate::region::{ChunkHeader, CompressionType, RegionHeader, RegionLocation};
use crate::Error;
use axolotl_nbt::binary::Binary;
use axolotl_nbt::serde_impl;
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::{GzDecoder, ZlibDecoder};

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub trait RegionFileType:
    for<'de> serde::Deserialize<'de> + serde::Serialize + Debug + Clone
{
    fn get_path() -> &'static str
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct RegionFile {
    pub(crate) file: File,
    pub(crate) region_header: RegionHeader,
}

impl RegionFile {
    pub fn replace(&mut self, mut file: File, initialized: bool) -> Result<(), Error> {
        if initialized {
            RegionHeader::replace_region_header(
                &mut file,
                &mut self.region_header.locations,
                &mut self.region_header.timestamps,
            )?;
        } else {
            self.region_header.initialize_and_zero(&mut file)?;
        }
        self.file = file;

        Ok(())
    }

    pub fn new(mut file: File, initialized: bool) -> Result<Self, Error> {
        if initialized {
            let region_header = RegionHeader::read_region_header(&mut file)?;
            Ok(Self {
                file,
                region_header,
            })
        } else {
            RegionHeader::initialize(&mut file)?;
            let region_header = RegionHeader::default();
            Ok(Self {
                file,
                region_header,
            })
        }
    }

    /// A Chunk with 0 length is considered empty
    #[inline]
    pub fn read_chunk_header(&mut self, location: &RegionLocation) -> Result<ChunkHeader, Error> {
        if location.0 == 0 {
            return Ok(ChunkHeader::default());
        }
        let calc_offset = location.calc_offset();
        self.file.seek(SeekFrom::Start(calc_offset as u64))?;

        let length = self.file.read_u32::<BigEndian>()?;
        let compression_type = self.file.read_u8()?;
        Ok(ChunkHeader {
            length,
            compression_type: compression_type.into(),
        })
    }

    pub fn read_chunk<FileType: RegionFileType>(
        &mut self,
        location: &RegionLocation,
    ) -> Result<Option<(ChunkHeader, FileType)>, Error> {
        let result = self.read_chunk_header(location)?;
        if result.length == 0 {
            return Ok(None);
        }

        let take = (&mut self.file).take((result.length - 1) as u64);
        #[cfg(feature = "log")]
        log::debug!("Compression Type {:?}", &result.compression_type);
        let value = match &result.compression_type {
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
        Ok(Some((result, value)))
    }
    pub fn read_chunk_in_place<FileType: RegionFileType>(
        &mut self,
        location: &RegionLocation,
        chunk: &mut FileType,
    ) -> Result<Option<ChunkHeader>, Error> {
        let result = self.read_chunk_header(location)?;
        if result.length == 0 {
            return Ok(None);
        }

        let take = (&mut self.file).take((result.length - 1) as u64);
        #[cfg(feature = "log")]
        log::debug!("Compression Type {:?}", &result.compression_type);
        match &result.compression_type {
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
        Ok(Some(result))
    }

    /// Saves the RegionHeader to the file
    pub fn save(&mut self) -> Result<(), Error> {
        self.region_header.write_region(&mut self.file)?;
        Ok(())
    }
}
