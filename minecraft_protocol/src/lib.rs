#![deny(deprecated_in_future, deprecated)]
extern crate core;

use std::fmt::Debug;
use std::io::{Read, Write};

use aes::Aes128;
use bytes::BytesMut;
pub use cfb_mode::cipher::AsyncStreamCipher;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod data;
pub mod java;
pub mod packets;
pub mod simple_handlers;

/// The Encryptor type used for Encrypting Packets
#[cfg(feature = "encryption")]
pub type Encryptor = cfb_mode::Encryptor<Aes128>;
/// The Decryptor type used for Decrypting Packets
#[cfg(feature = "encryption")]
pub type Decryptor = cfb_mode::Decryptor<Aes128>;

#[derive(Debug, Error)]
pub enum PacketWriteError {
    #[error("Failed to write value: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid Packet Type. (undefined behavior)")]
    InvalidPacketType,
    #[error("NBT Error {0}")]
    NBTError(#[from] nbt::Error),
    #[error("Failed to serialize value: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error("Failed to write value: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown packet id: {0}")]
    UnknownPacketId(i32),

    #[error("Invalid Data: {0}")]
    InvalidData(anyhow::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bound {
    ServerBound,
    ClientBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    Java(i32),
}

/// Generic Packet Handler
pub trait PacketHandler: Debug {
    fn set_compression(&mut self, _compression: CompressionSettings) {
        unimplemented!("This protocol does not support compression")
    }
    fn get_compression(&self) -> CompressionSettings {
        unimplemented!("This protocol does not support compression")
    }
}

/// As of Now we only support Zlib Compression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", content = "settings")]
pub enum CompressionSettings {
    #[default]
    None,
    Zlib {
        threshold: i32,
        compression_level: u32,
    },
}
pub trait PacketWriter: PacketHandler {
    type Buffer;
    /// What is being sent out
    type PacketOut: PacketContent;
    fn force_buffer_clear(&mut self);

    fn get_buffer(&mut self) -> &mut Self::Buffer;

    fn set_encryptor(&mut self, _encryptor: Encryptor) {
        unimplemented!("This protocol does not support encryption")
    }
    /// Writes the packet to the buffer. You are responsible for clearing the buffer
    fn write_packet(&mut self, packet: impl Into<Self::PacketOut>) -> Result<(), PacketWriteError>;
    fn send_packet<W: Write>(
        &mut self,
        packet: impl Into<Self::PacketOut>,
        writer: &mut W,
    ) -> Result<(), PacketWriteError>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketLength {
    LengthRead { length: i32, iterations: u8 },
    Incomplete,
}
impl ReadBufferType for BytesMut {
    fn len(&self) -> usize {
        self.len()
    }
}
pub trait ReadBufferType {
    fn len(&self) -> usize;
}
pub trait PacketReader: PacketHandler {
    /// What is coming in
    type PacketIn: PacketContent;
    type ReadBuffer: ReadBufferType;
    /// Not Implemented for Handshake + Status

    /// Not Implemented for Handshake + Status
    fn set_decryptor(&mut self, _decryptor: Decryptor) {
        unimplemented!("Decryptor not implemented for this protocol")
    }

    fn packet_len(&self) -> &PacketLength;
    /// The minimum numbers of bytes needed to read before attempt_packet_read should be called
    fn minimum_bytes_needed(&self) -> usize {
        if let PacketLength::LengthRead { length, .. } = self.packet_len() {
            let len = *length as usize;
            return if self.get_read_buffer_ref().len() >= len {
                0
            } else {
                len - self.get_read_buffer_ref().len()
            };
        } else {
            0
        }
    }

    fn attempt_packet_read(&mut self) -> Result<Option<Self::PacketIn>, PacketReadError>;
    fn get_read_buffer(&mut self) -> &mut Self::ReadBuffer;
    fn get_read_buffer_ref(&self) -> &Self::ReadBuffer;

    /// Forces the Buffer to clear.
    fn force_buffer_clear(&mut self);
}

pub trait Packet: Debug {
    type Content: PacketContent;
    /// The bytes of the packet are written to the writer skipping the converting from usize to varint
    fn write_packet_id<W: Write>(w: &mut W) -> Result<usize, PacketWriteError>;
    /// The Packet ID
    fn packet_id() -> i32;
    /// The Packet Bound
    fn bound() -> Bound;
    /// The Packet Stage
    fn stage() -> Stage;
    /// The Packet Protocol
    fn protocol() -> Protocol;
    /// Writes the packet to the writer + the packet id
    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError>;

    /// Reads the packet from the reader
    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError>;
    #[inline(always)]
    fn read_with_length<R: Read>(
        r: &mut R,
        _length: usize,
    ) -> Result<Self::Content, PacketReadError> {
        Self::read(r)
    }

    fn minimum_size() -> usize {
        0
    }
}
macro_rules! define_id_fns {
    ($id:literal) => {
        #[inline]
        fn write_packet_id<W: std::io::Write>(w: &mut W) -> Result<usize, crate::PacketWriteError> {
            let value = minecraft_protocol_macros::define_var_int!($id);
            w.write(&value).map_err(crate::PacketWriteError::IoError)
        }
        #[inline(always)]
        fn packet_id() -> i32 {
            $id
        }
    };
}

/// A Low Level Packet Writer + Reader
///
/// This is meant for reading and writing the packet Enums for each stage. This will not handle any form of encryption or compression.
///
/// It will assume all data in the buffer is for the current packet and will read till the end of the buffer.
pub trait PacketIO {
    type Type: PacketContent;
    fn handle_read<R: Read>(
        packed_id: i32,
        len: usize,
        reader: &mut R,
    ) -> Result<Self::Type, PacketReadError>;
    fn handle_write<W: Write>(packet: Self::Type, writer: &mut W) -> Result<(), PacketWriteError>;
}

pub(crate) use define_id_fns;

pub trait PacketContent: Clone + Debug {}
macro_rules! impl_packet_content {
    ($($t:ty),*) => {
        $(
            impl PacketContent for $t {}
        )*
    };
}
impl_packet_content!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String, bool);

impl PacketContent for () {}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use minecraft_protocol_macros::define_var_int;

    use crate::{Decryptor, Encryptor};

    #[test]
    pub fn test() {
        println!("{}", size_of::<Encryptor>());
        println!("{:?}", size_of::<Decryptor>());
        let v = define_var_int!(22645);
        for i in v {
            println!("{}", i);
        }
    }
}
