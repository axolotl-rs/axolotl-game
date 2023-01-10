use std::fmt::Debug;
use std::io::Write;

use bytes::BytesMut;

use crate::simple_handlers::{InternalPacketReader, InternalPacketWriter};
use crate::{
    CompressionSettings, PacketHandler, PacketIO, PacketLength, PacketReadError, PacketReader,
    PacketWriteError, PacketWriter,
};

#[derive(Debug, Clone)]
pub struct NonEncryptedPacketReader<IO: PacketIO> {
    pub phantom: std::marker::PhantomData<IO>,
    pub buffer: BytesMut,
    pub packet_len: PacketLength,
    pub compression: CompressionSettings,
}

impl<IO: PacketIO> Default for NonEncryptedPacketReader<IO> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            buffer: BytesMut::with_capacity(4096),
            packet_len: PacketLength::Incomplete,
            compression: CompressionSettings::default(),
        }
    }
}

impl<IO: PacketIO + Debug> PacketHandler for NonEncryptedPacketReader<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        self.compression = compression;
    }
    fn get_compression(&self) -> CompressionSettings {
        self.compression
    }
}

impl<IO: PacketIO + Debug> InternalPacketReader for NonEncryptedPacketReader<IO> {
    #[inline(always)]
    fn set_packet_length(&mut self, length: PacketLength) {
        self.packet_len = length;
    }
}
impl<IO: PacketIO + Debug> PacketReader for NonEncryptedPacketReader<IO> {
    type PacketIn = IO::Type;
    type ReadBuffer = BytesMut;

    fn packet_len(&self) -> &PacketLength {
        &self.packet_len
    }

    fn attempt_packet_read(&mut self) -> Result<Option<Self::PacketIn>, PacketReadError> {
        self.attempt_read::<IO>()
    }

    fn get_read_buffer(&mut self) -> &mut Self::ReadBuffer {
        &mut self.buffer
    }

    fn get_read_buffer_ref(&self) -> &Self::ReadBuffer {
        &self.buffer
    }

    fn force_buffer_clear(&mut self) {
        self.buffer.clear();
    }
}

#[derive(Debug, Clone)]
pub struct NonEncryptedPacketWriter<IO: PacketIO> {
    pub pending_buffer: Vec<u8>,
    pub compression: CompressionSettings,
    pub phantom: std::marker::PhantomData<IO>,
}

impl<IO: PacketIO> Default for NonEncryptedPacketWriter<IO> {
    fn default() -> Self {
        Self {
            pending_buffer: Vec::with_capacity(1024),
            compression: CompressionSettings::default(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<IO: PacketIO + Debug> InternalPacketWriter<IO> for NonEncryptedPacketWriter<IO> {}
impl<IO: PacketIO + Debug> PacketHandler for NonEncryptedPacketWriter<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        self.compression = compression;
    }
    fn get_compression(&self) -> CompressionSettings {
        self.compression
    }
}

impl<IO: PacketIO + Debug> PacketWriter for NonEncryptedPacketWriter<IO> {
    type Buffer = Vec<u8>;
    type PacketOut = IO::Type;

    fn force_buffer_clear(&mut self) {
        self.pending_buffer.clear();
    }

    fn get_buffer(&mut self) -> &mut Self::Buffer {
        &mut self.pending_buffer
    }

    fn write_packet(&mut self, packet: impl Into<Self::PacketOut>) -> Result<(), PacketWriteError> {
        self.internal_write(packet.into())?;
        Ok(())
    }

    fn send_packet<W: Write>(
        &mut self,
        packet: impl Into<Self::PacketOut>,
        writer: &mut W,
    ) -> Result<(), PacketWriteError> {
        self.internal_write(packet.into())?;
        writer.write_all(&self.pending_buffer)?;

        self.pending_buffer.clear();
        Ok(())
    }
}
