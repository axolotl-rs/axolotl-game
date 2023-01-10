use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::data::PacketDataType;

impl PacketDataType for f32 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_f32::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_f32::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}

impl PacketDataType for f64 {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_f64::<byteorder::BigEndian>()
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()>
    where
        Self: Sized,
    {
        writer.write_f64::<byteorder::BigEndian>(self)?;
        Ok(())
    }
}
