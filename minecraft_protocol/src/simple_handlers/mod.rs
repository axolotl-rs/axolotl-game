use std::io::{BufRead, Cursor, Write};
use std::mem;

use bytes::{Buf, BytesMut};
use flate2::bufread::GzDecoder;
use log::debug;

#[cfg(feature = "encryption")]
pub use encrypted::{EncryptedPacketReader, EncryptedPacketWriter};
pub use no_encryption::{NonEncryptedPacketReader, NonEncryptedPacketWriter};
#[cfg(feature = "encryption")]
pub use optional_encryption::{OptionalEncryptionReader, OptionalEncryptionWriter};

use crate::data::var_int::VarInt;
use crate::data::{var_int, PacketDataType};
use crate::{
    CompressionSettings, PacketIO, PacketLength, PacketReadError, PacketReader, PacketWriteError,
    PacketWriter, ReadBufferType,
};

#[cfg(feature = "encryption")]
mod encrypted;

mod no_encryption;
#[cfg(feature = "encryption")]
pub mod optional_encryption;

pub(crate) trait InternalPacketWriter<IO: PacketIO>: PacketWriter<Buffer = Vec<u8>> {
    fn internal_write(&mut self, packet: IO::Type) -> Result<(), PacketWriteError> {
        let mut header = [0u8; 6];
        let mut header_len = 0usize;
        self.get_buffer().write_all(&header)?;
        IO::handle_write(packet.into(), &mut self.get_buffer())?;
        let len_as_i32 = self.get_buffer()[6..].len() as i32;

        if let CompressionSettings::Zlib {
            threshold,
            compression_level,
        } = &self.get_compression()
        {
            if *threshold >= len_as_i32 {
                let compressed = Vec::with_capacity(self.get_buffer().len());
                let mut compressor = flate2::write::ZlibEncoder::new(
                    compressed,
                    flate2::Compression::new(*compression_level),
                );
                compressor.write_all(&mut self.get_buffer())?;
                let compressed = compressor.finish()?;

                header_len = var_int::inline::write(compressed.len() as i32, &mut header.as_mut())?;
                header_len = header_len
                    + var_int::inline::write(
                        self.get_buffer().len() as i32,
                        &mut header[header_len..].as_mut(),
                    )?;
                self.get_buffer().clear();
                self.get_buffer().write_all(&header[..header_len])?;
                self.get_buffer().write_all(&compressed)?;
                return Ok(());
            } else {
                header_len = var_int::inline::write(len_as_i32, &mut header.as_mut())? + 1;
            }
        } else {
            header_len = var_int::inline::write(len_as_i32, &mut header.as_mut())?;
        }
        self.get_buffer()
            .splice(0..6, header[..header_len].iter().cloned());

        Ok(())
    }
}
/// Internal trait for packet readers.
pub(crate) trait InternalPacketReader: PacketReader<ReadBuffer = BytesMut> {
    fn set_packet_length(&mut self, length: PacketLength);
    /// This function is used to attempt to read the packet.
    ///
    /// This is assuming the data in the buffer is decrypted
    fn attempt_read<IO: PacketIO>(&mut self) -> Result<Option<IO::Type>, PacketReadError> {
        if self.get_read_buffer_ref().is_empty() {
            return Ok(None);
        }
        let (packet_len, iterations) =
        // Check for a pre-existing packet length
            if let PacketLength::LengthRead { length, iterations } = self.packet_len() {
                (*length, *iterations as usize)
            } else {
                // If there is no pre-existing packet length, attempt to read one
                // Ensure that that data is available
                if self.get_read_buffer_ref().len() == 0 {
                    return Ok(None);
                }
                // Creates a cursor to read the data
                let mut cursor = Cursor::new(
                    self.get_read_buffer_ref().as_ref()[0..self.get_read_buffer_ref().len().min(4)]
                        .to_vec(),
                );
                // Read the packet length
                let packet_len_value = var_int::inline::read_with_iterations(&mut cursor);
                // If ok set the packet length and return
                if let Ok((len, iterations)) = packet_len_value {
                    self.set_packet_length(PacketLength::LengthRead {
                        length: len,
                        iterations: iterations as u8,
                    });

                    (len, iterations as usize)
                } else {
                    // The packet length is incomplete return and wait for more data
                    return Ok(None);
                }
            };
        // If we got a packet length. Check to see if we have the full packet. If not return None. and ensure we length for the entire packet.
        let packet_len_total = packet_len as usize + iterations;
        if self.get_read_buffer_ref().len() < packet_len_total {
            // Ensure the buffer is large enough to hold the entire packet

            let capacity = self.get_read_buffer_ref().capacity();
            if capacity < packet_len_total {
                self.get_read_buffer().reserve(packet_len_total - capacity);
            }
            return Ok(None);
        }
        // Check if compression is enabled. If so, decompress the packet.
        match self.get_compression() {
            CompressionSettings::Zlib { .. } => {
                let mut current_packet = self.get_read_buffer().split_to(packet_len_total).reader();

                let decompressed_size = VarInt::read(&mut current_packet)?;
                if decompressed_size.0 != 0 {
                    let mut decompressor = GzDecoder::new(current_packet);
                    let id = var_int::inline::read(&mut decompressor)?.0;
                    self.set_packet_length(PacketLength::Incomplete);

                    let packet =
                        IO::handle_read(id, decompressed_size.0 as usize, &mut decompressor)?;
                    return Ok(Some(packet));
                }
            }
            _ => {}
        }

        // Take the amount of bytes we need from the buffer and create a self
        let mut current_packet = self.get_read_buffer().split_to(packet_len_total).reader();
        // Consume the packet length
        current_packet.consume(iterations as usize);
        // Read Packet ID
        let id = var_int::inline::read(&mut current_packet)?.0;
        self.set_packet_length(PacketLength::Incomplete);

        // Read Packet
        let packet = IO::handle_read(id, packet_len as usize, &mut current_packet)?;
        // Clear Packet Length
        let mut current_packet = current_packet.into_inner();
        {
            current_packet.clear();
            if self.get_read_buffer().is_empty() {
                self.get_read_buffer().unsplit(current_packet);
            } else {
                let current_packet = mem::replace(self.get_read_buffer(), current_packet);
                self.get_read_buffer().unsplit(current_packet);
            }
        }
        Ok(Some(packet))
    }
}
