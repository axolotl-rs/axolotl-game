use crate::Error;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io::{Read, Write};

pub mod file;

#[derive(Debug)]
pub struct RegionWriter<Src: Debug> {
    pub(crate) src: Src,
}

impl<Src: Debug> RegionWriter<Src> {
    pub fn new(src: Src) -> Self {
        RegionWriter { src }
    }
    pub fn into_inner(self) -> Src {
        self.src
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Copy)]
pub struct RegionLocation(pub u32, pub u8);

impl Default for RegionLocation {
    #[inline]
    fn default() -> Self {
        Self(0, 0)
    }
}

impl RegionLocation {
    #[inline]
    pub fn calc_offset(&self) -> u64 {
        (self.0 * 4096) as u64
    }
}

impl Into<(u32, u8)> for RegionLocation {
    fn into(self) -> (u32, u8) {
        (self.0, self.1)
    }
}

impl From<(u32, u8)> for RegionLocation {
    fn from((offset, sector_count): (u32, u8)) -> Self {
        RegionLocation(offset, sector_count)
    }
}

#[derive(Debug, Clone)]
pub struct RegionHeader {
    /// The regions offsets and sizes
    pub locations: Vec<RegionLocation>,
    /// The timestamps
    pub timestamps: Vec<u32>,
}

impl Default for RegionHeader {
    fn default() -> Self {
        Self {
            locations: vec![RegionLocation::default(); 1024],
            timestamps: vec![0; 1024],
        }
    }
}

impl RegionHeader {
    pub fn initialize_and_zero<Writer: Write>(&mut self, writer: &mut Writer) -> Result<(), Error> {
        debug_assert!(self.locations.len() == 1024, "locations.len() == 1024");
        debug_assert!(self.locations.len() == 1024, "timestamps.len() == 1024");

        for header in self.locations.iter_mut() {
            header.0 = 0;
            header.1 = 0;
            writer.write_u24::<BigEndian>(0)?;
            writer.write_u8(0)?;
        }
        for timestamp in self.timestamps.iter_mut() {
            *timestamp = 0;
            writer.write_u32::<BigEndian>(0)?;
        }
        Ok(())
    }
    pub fn initialize<Writer: Write>(writer: &mut Writer) -> Result<(), Error> {
        for _ in 0..1024 {
            writer.write_u24::<BigEndian>(0)?;
            writer.write_u8(0)?;
        }
        for _ in 0..1024 {
            writer.write_u32::<BigEndian>(0)?;
        }
        Ok(())
    }
    /// This function will clear the header
    pub fn write_region<Writer: Write>(&self, writer: &mut Writer) -> Result<(), Error> {
        if self.locations.len() != 1024 {
            return Err(Error::InvalidChunkHeader("locations"));
        }
        if self.timestamps.len() != 1024 {
            return Err(Error::InvalidChunkHeader("timestamps"));
        }
        for location in self.locations.iter() {
            writer.write_u24::<BigEndian>(location.0)?;
            writer.write_u8(location.1)?;
        }
        for timestamp in self.timestamps.iter() {
            writer.write_u32::<BigEndian>(*timestamp)?;
        }
        Ok(())
    }

    pub fn read_region_header<Reader: Read>(reader: &mut Reader) -> Result<RegionHeader, Error> {
        let mut locations = Vec::with_capacity(1024);
        let mut timestamps = Vec::with_capacity(1024);
        for _ in 0..1024 {
            let offset = reader.read_u24::<BigEndian>()?;
            let sector_count = reader.read_u8()?;
            locations.push(RegionLocation(offset, sector_count));
        }
        for _ in 0..1024 {
            let timestamp = reader.read_u32::<BigEndian>()?;
            timestamps.push(timestamp);
        }
        Ok(RegionHeader {
            locations,
            timestamps,
        })
    }
    /// Replaces the data in the header with the data from the reader
    pub fn replace_region_header<Reader: Read>(
        reader: &mut Reader,
        locations: &mut Vec<RegionLocation>,
        timestamps: &mut Vec<u32>,
    ) -> Result<(), Error> {
        debug_assert!(locations.len() == 1024, "locations.len() == 1024");
        debug_assert!(timestamps.len() == 1024, "timestamps.len() == 1024");

        for header in locations {
            header.0 = reader.read_u24::<BigEndian>()?;
            header.1 = reader.read_u8()?;
        }
        for timestamp in timestamps {
            *timestamp = reader.read_u32::<BigEndian>()?;
        }
        Ok(())
    }
    #[inline(always)]
    pub fn get_index(v: impl Into<(i32, i32)>) -> i32 {
        let (x, z) = v.into();
        ((x % 32) + (z % 32) * 32) * 4
    }
    pub fn get_chunk_location(&self, v: impl Into<(i32, i32)>) -> Option<&RegionLocation> {
        self.locations.get(Self::get_index(v) as usize)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChunkHeader {
    pub length: u32,
    pub compression_type: CompressionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum CompressionType {
    Gzip,
    Zlib,
    #[default]
    Uncompressed,
    Custom(u8),
}

impl From<u8> for CompressionType {
    fn from(data: u8) -> Self {
        match data {
            3 => CompressionType::Uncompressed,
            1 => CompressionType::Gzip,
            2 => CompressionType::Zlib,
            _ => CompressionType::Custom(data),
        }
    }
}
