use cfb_mode::cipher::KeyIvInit;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use minecraft_protocol::java::v_761::play::ClientIO;
use minecraft_protocol::packets::play::client::ClientBoundPlay::Disconnect;
use minecraft_protocol::packets::play::client::{ClientBoundPlay, DisconnectPacket};
use minecraft_protocol::simple_handlers::{EncryptedPacketWriter, NonEncryptedPacketWriter};
use minecraft_protocol::{CompressionSettings, Encryptor, PacketHandler, PacketWriter};

fn write_packet(mut target: Vec<u8>, mut writer: impl PacketWriter<PacketOut = ClientBoundPlay>) {
    let packet = Disconnect(DisconnectPacket("Hello world!".to_string()));

    writer.send_packet(packet, &mut target).unwrap();
}

pub fn no_encryption(c: &mut Criterion) {
    c.bench_function("write_no_encryption_uncompressed_packet", |b| {
        b.iter_batched(
            || {
                let writer = NonEncryptedPacketWriter::<ClientIO>::default();
                let buffer = Vec::with_capacity(1024);
                (writer, buffer)
            },
            |(writer, buffer)| write_packet(black_box(buffer), black_box(writer)),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("write_no_encryption_compressed_packet", |b| {
        b.iter_batched(
            || {
                let mut writer = NonEncryptedPacketWriter::<ClientIO>::default();
                writer.set_compression(CompressionSettings::Zlib {
                    threshold: 0,
                    compression_level: 6,
                });
                let buffer = Vec::with_capacity(1024);
                (writer, buffer)
            },
            |(writer, buffer)| write_packet(black_box(buffer), black_box(writer)),
            criterion::BatchSize::SmallInput,
        )
    });
}
fn encryptor() -> Encryptor {
    Encryptor::new_from_slices(&[0; 16], &[0; 16]).expect("Failed to create encryptor")
}
pub fn encrypted(c: &mut Criterion) {
    let encryptor = encryptor();
    c.bench_function("write_encrypted_uncompressed_packet", |b| {
        b.iter_batched(
            || {
                let writer = EncryptedPacketWriter::<ClientIO>::new(encryptor.clone());
                let buffer = Vec::with_capacity(1024);
                (writer, buffer)
            },
            |(writer, buffer)| write_packet(black_box(buffer), black_box(writer)),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("write_encrypted_compressed_packet", |b| {
        b.iter_batched(
            || {
                let mut writer = EncryptedPacketWriter::<ClientIO>::new(encryptor.clone());
                writer.set_compression(CompressionSettings::Zlib {
                    threshold: 0,
                    compression_level: 6,
                });
                let buffer = Vec::with_capacity(1024);
                (writer, buffer)
            },
            |(writer, buffer)| write_packet(black_box(buffer), black_box(writer)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, no_encryption, encrypted);
criterion_main!(benches);
