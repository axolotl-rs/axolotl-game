use std::fmt::Debug;
use std::io::Write;

use bytes::BytesMut;
use cfb_mode::cipher::AsyncStreamCipher;

use crate::simple_handlers::{InternalPacketReader, InternalPacketWriter};
use crate::{
    CompressionSettings, Decryptor, Encryptor, PacketHandler, PacketIO, PacketLength,
    PacketReadError, PacketReader, PacketWriteError, PacketWriter,
};

#[derive(Debug, Clone)]
pub struct EncryptedPacketReader<IO: PacketIO> {
    pub phantom: std::marker::PhantomData<IO>,
    pub buffer: BytesMut,
    pub packet_len: PacketLength,
    pub compression: CompressionSettings,
    pub decryptor: Decryptor,
    pub last_decrypted_at: usize,
}

impl<IO: PacketIO + Debug> EncryptedPacketReader<IO> {
    pub fn new(decryptor: Decryptor) -> Self {
        Self {
            phantom: Default::default(),
            buffer: BytesMut::with_capacity(4096),
            packet_len: PacketLength::Incomplete,
            compression: Default::default(),
            decryptor,
            last_decrypted_at: 0,
        }
    }
    /// Decrypts the area of the buffer that has not been decrypted yet.
    fn decrypt(&mut self) {
        let decrypt_buffer = &mut self.buffer.as_mut()[self.last_decrypted_at..];
        self.decryptor.clone().decrypt(decrypt_buffer);
        self.last_decrypted_at = decrypt_buffer.len();
    }
}
impl<IO: PacketIO + Debug> PacketHandler for EncryptedPacketReader<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        self.compression = compression;
    }
    fn get_compression(&self) -> CompressionSettings {
        self.compression
    }
}

impl<IO: PacketIO + Debug> InternalPacketReader for EncryptedPacketReader<IO> {
    fn set_packet_length(&mut self, length: PacketLength) {
        self.packet_len = length;
    }
}
impl<IO: PacketIO + Debug> PacketReader for EncryptedPacketReader<IO> {
    type PacketIn = IO::Type;
    type ReadBuffer = BytesMut;

    fn set_decryptor(&mut self, decryptor: Decryptor) {
        self.decryptor = decryptor;
    }

    fn packet_len(&self) -> &PacketLength {
        &self.packet_len
    }

    fn attempt_packet_read(&mut self) -> Result<Option<Self::PacketIn>, PacketReadError> {
        self.decrypt();
        self.attempt_read::<IO>()
    }

    fn get_read_buffer(&mut self) -> &mut Self::ReadBuffer {
        &mut self.buffer
    }
    fn get_read_buffer_ref(&self) -> &Self::ReadBuffer {
        &self.buffer
    }

    fn force_buffer_clear(&mut self) {
        self.last_decrypted_at = 0;
        self.packet_len = PacketLength::Incomplete;
        self.buffer.clear();
    }
}

#[derive(Debug, Clone)]
pub struct EncryptedPacketWriter<IO: PacketIO> {
    pub pending_buffer: Vec<u8>,
    pub compression: CompressionSettings,
    pub encryptor: crate::Encryptor,
    pub phantom: std::marker::PhantomData<IO>,
}

impl<IO: PacketIO + Debug> EncryptedPacketWriter<IO> {
    pub fn new(encryptor: Encryptor) -> Self {
        Self {
            phantom: Default::default(),
            pending_buffer: Vec::with_capacity(1024),
            compression: Default::default(),
            encryptor,
        }
    }
}

impl<IO: PacketIO + Debug> InternalPacketWriter<IO> for EncryptedPacketWriter<IO> {}
impl<IO: PacketIO + Debug> PacketHandler for EncryptedPacketWriter<IO> {
    fn set_compression(&mut self, compression: CompressionSettings) {
        self.compression = compression;
    }
    fn get_compression(&self) -> CompressionSettings {
        self.compression
    }
}

impl<IO: PacketIO + Debug> PacketWriter for EncryptedPacketWriter<IO> {
    type Buffer = Vec<u8>;
    type PacketOut = IO::Type;

    fn force_buffer_clear(&mut self) {
        self.pending_buffer.clear();
    }

    fn get_buffer(&mut self) -> &mut Self::Buffer {
        &mut self.pending_buffer
    }

    fn set_encryptor(&mut self, encryptor: Encryptor) {
        self.encryptor = encryptor;
    }

    fn write_packet(&mut self, packet: impl Into<Self::PacketOut>) -> Result<(), PacketWriteError> {
        // Call the internal send packet function
        self.internal_write(packet.into())?;
        // Push the pending buffer into the framed buffer

        self.encryptor.clone().encrypt(&mut self.pending_buffer);

        Ok(())
    }

    fn send_packet<W: Write>(
        &mut self,
        packet: impl Into<Self::PacketOut>,
        writer: &mut W,
    ) -> Result<(), PacketWriteError> {
        // Call the internal send packet function
        self.internal_write(packet.into())?;
        // Push the pending buffer into the framed buffer

        self.encryptor.clone().encrypt(&mut self.pending_buffer);

        writer.write_all(&self.pending_buffer)?;
        writer.flush()?;

        self.pending_buffer.clear();

        Ok(())
    }
}
