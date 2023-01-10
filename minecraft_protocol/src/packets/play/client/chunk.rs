use std::io;
use std::io::{Read, Write};
use std::slice::Iter;

use minecraft_protocol_macros::PacketContentType;

use crate::data::var_int::inline::get_size;
use crate::data::var_int::VarInt;
use crate::data::{NBTOrByteArray, PacketDataType};
use crate::PacketContent;

pub type BitSet = Vec<i64>;
/// \[Number Of Arrays] \[Arrays]
///
/// Array is \[Array Length] \[Array of Bytes]
///
/// Each Array should be 2048 bytes long
pub type Light = Vec<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    pub x: i8,
    pub z: i8,
    pub y: i16,
    pub block_type: VarInt,
    pub data: NBTOrByteArray,
}

impl BlockEntity {
    /// XZ are a packed i8 4 bits each
    pub fn pack_xz(&self) -> i8 {
        ((self.x & 15) << 4) | (self.z & 15)
    }
}

/// Should be implemented on a Paletted Container structure
///
/// Will handle reading and writing the structure in the packet format
///
///
/// Uses the following format:
/// - 1 byte for bits per entry
/// - VarInt for palette length
/// - palette length * VarInt for palette entries
/// - VarInt for array length
/// - An Array of i64s for the data
pub trait PalettePacketContent {
    // The datatype this palette holds
    type Type: GetVanillaId;
    type Palette<'iter>: Iterator<Item = &'iter Self::Type> + ExactSizeIterator
    where
        Self: 'iter;
    type Indexes<'iter>: Iterator<Item = &'iter i64> + ExactSizeIterator
    where
        Self: 'iter;

    fn get_palette(&self) -> Self::Palette<'_>;

    fn get_indexes(&self) -> Self::Indexes<'_>;

    fn get_bits_per_entry(&self) -> u8;
    /// Calculates the size of the packet data
    fn calculate_buffer_size(&self) -> usize {
        let mut total_size = 1
            + get_size(self.get_palette().len() as i32) as usize
            + get_size(self.get_indexes().len() as i32) as usize
            + (8 * self.get_indexes().len());
        for entry in self.get_palette() {
            total_size += get_size(entry.get_vanilla_id() as i32) as usize;
        }
        total_size
    }
    fn write_palette<Writer: Write>(&self, writer: &mut Writer) -> io::Result<()> {
        self.get_bits_per_entry().write(writer)?;
        let palette = self.get_palette();
        VarInt(palette.len() as i32).write(writer)?;
        for block in palette {
            block.get_vanilla_id().write(writer)?;
        }
        let indexes = self.get_indexes();
        VarInt(indexes.len() as i32).write(writer)?;
        for index in indexes {
            index.write(writer)?;
        }
        Ok(())
    }
}

pub trait PalettePacketContentReader: PalettePacketContent {
    fn new(bits_per_entry: u8, blocks: Vec<i32>, indexes: Vec<i64>) -> Self;

    fn read_palette<Reader: Read>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized,
    {
        let bits_per_entry = u8::read(reader)?;

        let palette_len = VarInt::read(reader)?.0 as usize;
        let mut palette = Vec::with_capacity(palette_len);
        for _ in 0..palette_len {
            palette.push(VarInt::read(reader)?.0);
        }

        let indexes_len = VarInt::read(reader)?.0 as usize;
        let mut indexes = Vec::with_capacity(indexes_len);
        for _ in 0..indexes_len {
            indexes.push(i64::read(reader)?);
        }

        Ok(Self::new(bits_per_entry, palette, indexes))
    }
}

pub struct GeneralPalettedContainer {
    pub bits_per_entry: u8,
    pub palette: Vec<i32>,
    pub indexes: Vec<i64>,
}

impl PalettePacketContent for GeneralPalettedContainer {
    type Type = i32;
    type Palette<'iter> = Iter<'iter, i32>;
    type Indexes<'iter> = Iter<'iter, i64>;

    fn get_palette(&self) -> Self::Palette<'_> {
        self.palette.iter()
    }

    fn get_indexes(&self) -> Self::Indexes<'_> {
        self.indexes.iter()
    }
    fn get_bits_per_entry(&self) -> u8 {
        self.bits_per_entry
    }
}

impl PalettePacketContentReader for GeneralPalettedContainer {
    fn new(bits_per_entry: u8, blocks: Vec<i32>, indexes: Vec<i64>) -> Self {
        Self {
            bits_per_entry,
            palette: blocks,
            indexes,
        }
    }
}

pub trait GetVanillaId {
    /// Returns the Vanilla ID for the type
    ///
    /// Note For Blocks this should be the BlockState ID
    fn get_vanilla_id(&self) -> i32;
}

impl GetVanillaId for i32 {
    #[inline(always)]
    fn get_vanilla_id(&self) -> i32 {
        *self
    }
}

#[derive(Debug, Clone, PartialEq, PacketContentType, Default)]
pub struct LightPacket {
    pub trust_edges: bool,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light: Light,
    pub block_light: Light,
}
#[derive(Debug, Clone, PartialEq, PacketContentType)]
pub struct UpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub light: LightPacket,
}

#[derive(Debug, Clone, PartialEq, PacketContentType)]
pub struct ChunkPacket {
    pub height_map: NBTOrByteArray,
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone, PartialEq, PacketContentType)]
pub struct ChunkDataAndLight {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkPacket,
    pub light: LightPacket,
}
