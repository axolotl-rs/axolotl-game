use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::data::PacketDataType;

impl PacketDataType for u8 {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u8()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<()>
    where
        Self: Sized,
    {
        write.write_u8(self)?;
        Ok(())
    }
}

impl PacketDataType for u16 {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u16::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<()>
    where
        Self: Sized,
    {
        write.write_u16::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}

impl PacketDataType for u32 {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u32::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<()>
    where
        Self: Sized,
    {
        write.write_u32::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}

impl PacketDataType for u64 {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_u64::<byteorder::BigEndian>()
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<()>
    where
        Self: Sized,
    {
        write.write_u64::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}
