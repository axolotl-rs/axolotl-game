use std::fmt::Debug;
use std::io::Write;
use std::ptr;

use bytes::BytesMut;

use crate::simple_handlers::encrypted::{EncryptedPacketReader, EncryptedPacketWriter};
use crate::simple_handlers::no_encryption::NonEncryptedPacketReader;
use crate::simple_handlers::NonEncryptedPacketWriter;
use crate::{
    CompressionSettings, Decryptor, Encryptor, PacketHandler, PacketIO, PacketLength,
    PacketReadError, PacketReader, PacketWriteError, PacketWriter,
};

#[derive(Debug, Clone)]
pub enum OptionalEncryptionReader<IO: PacketIO> {
    Encrypted(EncryptedPacketReader<IO>),
    NoEncryption(NonEncryptedPacketReader<IO>),
}

impl<IO: PacketIO + Debug> Default for OptionalEncryptionReader<IO> {
    fn default() -> Self {
        OptionalEncryptionReader::NoEncryption(NonEncryptedPacketReader::<IO>::default())
    }
}
impl<IO: PacketIO + Debug> PacketHandler for OptionalEncryptionReader<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.set_compression(compression),
            OptionalEncryptionReader::NoEncryption(reader) => reader.set_compression(compression),
        }
    }
}

impl<IO: PacketIO + Debug> PacketReader for OptionalEncryptionReader<IO> {
    type PacketIn = IO::Type;
    type ReadBuffer = BytesMut;

    ///
    /// If the reader is encrypted it will set the decryptor
    ///
    /// If the reader is not encrypted it replace the reader with an encrypted reader.
    fn set_decryptor(&mut self, decryptor: Decryptor) {
        match self {
            OptionalEncryptionReader::Encrypted(ok) => {
                ok.set_decryptor(decryptor);
            }
            OptionalEncryptionReader::NoEncryption(v) => unsafe {
                let value = ptr::read(v);
                ptr::write(
                    self,
                    OptionalEncryptionReader::Encrypted(EncryptedPacketReader {
                        phantom: value.phantom,
                        buffer: value.buffer,
                        packet_len: value.packet_len,
                        compression: value.compression,
                        decryptor,
                        last_decrypted_at: 0,
                    }),
                );
            },
        }
    }

    fn packet_len(&self) -> &PacketLength {
        match self {
            OptionalEncryptionReader::Encrypted(ok) => ok.packet_len(),
            OptionalEncryptionReader::NoEncryption(v) => v.packet_len(),
        }
    }

    fn minimum_bytes_needed(&self) -> usize {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.minimum_bytes_needed(),
            OptionalEncryptionReader::NoEncryption(reader) => reader.minimum_bytes_needed(),
        }
    }

    fn attempt_packet_read(&mut self) -> Result<Option<Self::PacketIn>, PacketReadError> {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.attempt_packet_read(),
            OptionalEncryptionReader::NoEncryption(reader) => reader.attempt_packet_read(),
        }
    }

    fn get_read_buffer(&mut self) -> &mut Self::ReadBuffer {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.get_read_buffer(),
            OptionalEncryptionReader::NoEncryption(reader) => reader.get_read_buffer(),
        }
    }

    fn get_read_buffer_ref(&self) -> &Self::ReadBuffer {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.get_read_buffer_ref(),
            OptionalEncryptionReader::NoEncryption(reader) => reader.get_read_buffer_ref(),
        }
    }

    fn force_buffer_clear(&mut self) {
        match self {
            OptionalEncryptionReader::Encrypted(reader) => reader.force_buffer_clear(),
            OptionalEncryptionReader::NoEncryption(reader) => reader.force_buffer_clear(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OptionalEncryptionWriter<IO: PacketIO> {
    Encrypted(EncryptedPacketWriter<IO>),
    NoEncryption(NonEncryptedPacketWriter<IO>),
}

impl<IO: PacketIO + Debug> Default for OptionalEncryptionWriter<IO> {
    fn default() -> Self {
        OptionalEncryptionWriter::NoEncryption(NonEncryptedPacketWriter::<IO>::default())
    }
}
impl<IO: PacketIO + Debug> PacketHandler for OptionalEncryptionWriter<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        match self {
            OptionalEncryptionWriter::Encrypted(writer) => writer.set_compression(compression),
            OptionalEncryptionWriter::NoEncryption(writer) => writer.set_compression(compression),
        }
    }
}

impl<IO: PacketIO + Debug> PacketWriter for OptionalEncryptionWriter<IO> {
    type Buffer = Vec<u8>;
    type PacketOut = IO::Type;
    fn force_buffer_clear(&mut self) {
        match self {
            OptionalEncryptionWriter::Encrypted(reader) => reader.force_buffer_clear(),
            OptionalEncryptionWriter::NoEncryption(reader) => reader.force_buffer_clear(),
        }
    }

    fn get_buffer(&mut self) -> &mut Self::Buffer {
        match self {
            OptionalEncryptionWriter::Encrypted(reader) => reader.get_buffer(),
            OptionalEncryptionWriter::NoEncryption(reader) => reader.get_buffer(),
        }
    }

    ///
    /// If the reader is encrypted it will set the encryptor
    ///
    /// If the reader is not encrypted it replace the reader with an encrypted reader.
    /// This uses unsafe to temporarily move the reference so we can move all the buffers over.
    fn set_encryptor(&mut self, encryptor: Encryptor) {
        match self {
            OptionalEncryptionWriter::Encrypted(ok) => {
                ok.set_encryptor(encryptor);
            }
            OptionalEncryptionWriter::NoEncryption(v) => unsafe {
                let value = ptr::read(v);
                ptr::write(
                    self,
                    OptionalEncryptionWriter::Encrypted(EncryptedPacketWriter {
                        pending_buffer: value.pending_buffer,
                        compression: value.compression,
                        encryptor,
                        phantom: value.phantom,
                    }),
                );
            },
        }
    }

    fn write_packet(&mut self, packet: impl Into<Self::PacketOut>) -> Result<(), PacketWriteError> {
        match self {
            OptionalEncryptionWriter::Encrypted(reader) => reader.write_packet(packet),
            OptionalEncryptionWriter::NoEncryption(reader) => reader.write_packet(packet),
        }
    }

    fn send_packet<W: Write>(
        &mut self,
        packet: impl Into<Self::PacketOut>,
        w: &mut W,
    ) -> Result<(), PacketWriteError> {
        match self {
            OptionalEncryptionWriter::Encrypted(writer) => writer.send_packet(packet, w),
            OptionalEncryptionWriter::NoEncryption(writer) => writer.send_packet(packet, w),
        }
    }
}
