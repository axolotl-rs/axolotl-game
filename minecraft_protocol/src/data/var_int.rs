use std::fmt::{Debug, Display};
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::str::FromStr;

use bytemuck_derive::{Pod, Zeroable};
use nbt::Blob;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use minecraft_protocol_macros::define_var_int;

use crate::data::PacketDataType;

pub const ZERO: [u8; 1] = define_var_int!(0);
#[derive(Debug, Clone, Error)]
#[error("The VarInt is too big to be read")]
pub struct VarIntTooLong;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd, Zeroable, Pod,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct VarInt(pub i32);

impl PacketDataType for VarInt {
    fn read<R: Read>(buf: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        inline::read(buf)
    }

    fn write<W: Write>(self, write: &mut W) -> std::io::Result<()> {
        inline::write(self, write)?;
        Ok(())
    }
}

impl FromStr for VarInt {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(VarInt)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl PartialEq<i32> for VarInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Allows you to inline the VarInt read and write into the packet
/// It is a very marginal performance difference but could be worth it
/// https://nnethercote.github.io/perf-book/inlining.html
pub mod inline {
    use std::io::{Read, Write};

    use crate::data::var_int::VarInt;

    #[inline(always)]
    pub fn read_with_iterations<R: Read>(buf: &mut R) -> std::io::Result<(i32, u32)> {
        let mut number_of_reads = 0;
        let mut result = 0;
        let mut byte = [0u8];
        loop {
            buf.read_exact(&mut byte)?;
            let read = byte[0];

            let value = i32::from(read & 0x7F);
            result |= value.overflowing_shl(7 * number_of_reads).0;

            number_of_reads += 1;
            if number_of_reads > 5 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too long",
                ));
            }
            if read & 0x80 == 0 {
                break;
            }
        }
        Ok((result, number_of_reads))
    }

    #[inline(always)]
    #[allow(unused_variables, unused_assignments)]
    pub fn get_size(mut number: i32) -> u8 {
        let mut iterations = 0;
        loop {
            let mut temp = (number & 0x7F) as u8;
            number >>= 7;
            if number != 0 {
                temp |= 0x80;
            }

            iterations += 1;
            if number == 0 {
                break;
            }
        }
        iterations
    }

    #[inline(always)]
    pub fn read<R: Read>(buf: &mut R) -> std::io::Result<VarInt> {
        let mut number_of_reads = 0;
        let mut result = 0;
        let mut byte = [0u8];
        loop {
            buf.read_exact(&mut byte)?;
            let read = byte[0];

            let value = i32::from(read & 0x7F);
            result |= value.overflowing_shl(7 * number_of_reads).0;

            number_of_reads += 1;
            if number_of_reads > 5 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too long",
                ));
            }
            if read & 0x80 == 0 {
                break;
            }
        }
        Ok(VarInt(result))
    }

    #[inline(always)]
    pub fn write<W: Write + ?Sized, VI: Into<i32>>(
        var_int: VI,
        write: &mut W,
    ) -> std::io::Result<usize> {
        let mut x = var_int.into();
        let mut iterations = 0;
        loop {
            let mut temp = (x & 0x7F) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0x80;
            }

            write.write_all(&[temp])?;

            iterations += 1;
            if x == 0 {
                break;
            }
        }
        Ok(iterations)
    }
}

impl PacketDataType for Blob {
    fn read<Reader: Read>(reader: &mut Reader) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        nbt::from_reader(reader).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
    }

    fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<()> {
        nbt::to_writer(writer, &self, None).map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }
}
