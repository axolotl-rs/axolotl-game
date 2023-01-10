use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::data::PacketDataType;

impl PacketDataType for i8 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i8()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_i8(self)?;
        Ok(())
    }
}

impl PacketDataType for i16 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i16::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_i16::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}

impl PacketDataType for i32 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i32::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_i32::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}

impl PacketDataType for i64 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i64::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_i64::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}
