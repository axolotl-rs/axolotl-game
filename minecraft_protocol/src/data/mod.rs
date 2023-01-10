use std::borrow::Cow;
use std::io;
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use nbt::Blob;

use uuid::Uuid;

use crate::data::var_int::VarInt;

pub mod fpoints;
pub mod sints;
pub mod uints;
pub mod var_int;

pub trait Position {
    fn into_single_long(self) -> i64;

    fn from_single_long(long: i64) -> Self;
}

macro_rules! define_into_position {
    ($t:ty) => {
        impl Position for ($t, $t, $t) {
            fn into_single_long(self) -> i64 {
                let (x, y, z) = self;
                (x as i64 & 0x3FFFFFF) << 38 | (z as i64 & 0x3FFFFFF) << 12 | (y as i64 & 0xFFF)
            }

            fn from_single_long(long: i64) -> Self {
                let x = (long >> 38) & 0x3FFFFFF;
                let z = (long >> 12) & 0x3FFFFFF;
                let y = long & 0xFFF;
                (x as $t, y as $t, z as $t)
            }
        }
    };
}
define_into_position!(i32);
define_into_position!(i64);
define_into_position!(u32);
define_into_position!(u64);

pub fn into_position(x: i32, y: i32, z: i32) -> i64 {
    ((x as i64 & 0x3FFFFFF) << 38) | ((z as i64 & 0x3FFFFFF) << 12) | (y as i64 & 0xFFF)
}

/// Implemented for custom data types in packets
pub trait PacketDataType {
    fn read<Reader: Read>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized;
    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()>;
}
#[derive(Debug, Clone, PartialEq)]
pub enum NBTOrByteArray {
    NBT(Blob),
    ByteArray(Vec<u8>),
}

impl PacketDataType for NBTOrByteArray {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        nbt::from_reader(reader)
            .map(NBTOrByteArray::NBT)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()> {
        match self {
            Self::NBT(nbt) => {
                nbt::to_writer(writer, &nbt, None)?;
            }
            Self::ByteArray(bytes) => {
                writer.write_all(&bytes)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedPosition(pub u64);

impl<T: ?Sized> PacketDataType for Cow<'_, T>
where
    T: PacketDataType + Clone,
{
    fn read<Reader: Read>(reader: &mut Reader) -> io::Result<Self> {
        Ok(Cow::Owned(T::read(reader)?))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()> {
        self.into_owned().write(writer)
    }
}

impl PacketDataType for &'_ str {
    fn read<Reader: Read>(_reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized,
    {
        panic!("Cannot read &str from reader. Please use String instead")
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()> {
        let size = self.as_bytes().len();
        VarInt(size as i32).write(writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}
impl PacketDataType for PackedPosition {
    fn read<Reader: Read>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized,
    {
        Ok(PackedPosition(reader.read_u64::<BigEndian>()?))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.0)?;
        Ok(())
    }
}

impl PacketDataType for bool {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let i = buf.read_u8()?;
        Ok(i != 0)
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_u8(if self { 1 } else { 0 })?;
        Ok(())
    }
}

impl PacketDataType for String {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::read(reader)?;
        let mut buf = Vec::with_capacity(len.0 as usize);
        reader.take(len.0 as u64).read_to_end(&mut buf)?;
        Ok(String::from_utf8_lossy(buf.as_ref()).into_owned())
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        let len = VarInt(self.len() as i32);
        VarInt::write(len, writer)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: PacketDataType> PacketDataType for Vec<T> {
    fn read<Reader: Read>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::read(reader)?;
        let mut vec = Vec::with_capacity(len.0 as usize);
        for _ in 0..len.0 {
            vec.push(T::read(reader)?);
        }
        Ok(vec)
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()> {
        let len = VarInt(self.len() as i32);
        VarInt::write(len, writer)?;
        for item in self {
            T::write(item, writer)?;
        }
        Ok(())
    }
}

impl PacketDataType for Uuid {
    fn read<Reader: Read>(reader: &mut Reader) -> io::Result<Self>
    where
        Self: Sized,
    {
        let value = reader.read_u128::<BigEndian>()?;
        Ok(Uuid::from_u128(value))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> io::Result<()> {
        let (first, second) = self.as_u64_pair();
        writer.write_u64::<BigEndian>(first)?;
        writer.write_u64::<BigEndian>(second)?;
        Ok(())
    }
}
