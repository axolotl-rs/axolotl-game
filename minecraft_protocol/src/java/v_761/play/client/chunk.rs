use std::io::{Read, Write};

use minecraft_protocol_macros::PacketImplDebug;

use crate::data::var_int::VarInt;
use crate::data::PacketDataType;
use crate::java::define_packet;
use crate::packets::play::client::chunk::{
    BlockEntity, ChunkDataAndLight, ChunkPacket, LightPacket, UpdateLightPacket,
};
use crate::Protocol;
use crate::Protocol::Java;
use crate::{Bound, Packet, PacketReadError, PacketWriteError, Stage};

pub const MAX_SIZE: usize = 0x200000;

#[derive(PacketImplDebug)]
pub struct ClientBoundChunkDataImpl;

impl Packet for ClientBoundChunkDataImpl {
    define_packet!(
        ChunkDataAndLight,
        0x20,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.chunk_x.write(w)?;
        content.chunk_z.write(w)?;
        let chunk_data = content.chunk_data;
        chunk_data.height_map.write(w)?;
        VarInt(chunk_data.data.len() as i32).write(w)?;
        w.write_all(&chunk_data.data)?;
        VarInt(chunk_data.block_entities.len() as i32).write(w)?;
        for block_entity in chunk_data.block_entities {
            let xz = ((block_entity.x & 15) << 4) | (block_entity.z & 15);
            xz.write(w)?;
            block_entity.y.write(w)?;
            block_entity.block_type.write(w)?;
            block_entity.data.write(w)?;
        }
        write_light_content(content.light, w)
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        let chunk_x = PacketDataType::read(r)?;
        let chunk_z = PacketDataType::read(r)?;
        let chunk_data = {
            let height_map = PacketDataType::read(r)?;
            let data_len = VarInt::read(r)?.0 as usize;
            let mut data = vec![0; data_len];
            r.read_exact(&mut data)?;
            let block_entities_len = VarInt::read(r)?.0 as usize;
            let mut block_entities = Vec::with_capacity(block_entities_len);
            for _ in 0..block_entities_len {
                let xz: i8 = PacketDataType::read(r)?;
                let y = PacketDataType::read(r)?;
                let block_type = PacketDataType::read(r)?;
                let data = PacketDataType::read(r)?;
                block_entities.push(BlockEntity {
                    x: ((xz >> 4) & 15),
                    z: (xz & 15),
                    y,
                    block_type,
                    data,
                });
            }
            ChunkPacket {
                height_map,
                data,
                block_entities,
            }
        };
        let light = read_light_content(r)?;
        Ok(ChunkDataAndLight {
            chunk_x,
            chunk_z,
            chunk_data,
            light,
        })
    }
}

#[derive(PacketImplDebug)]
pub struct ClientBoundLightUpdateImpl;
impl Packet for ClientBoundLightUpdateImpl {
    define_packet!(
        UpdateLightPacket,
        0x23,
        Bound::ClientBound,
        Stage::Play,
        Java(761)
    );

    fn write<W: Write>(content: Self::Content, w: &mut W) -> Result<(), PacketWriteError> {
        Self::write_packet_id(w)?;
        content.chunk_x.write(w)?;
        content.chunk_z.write(w)?;
        write_light_content(content.light, w)
    }

    fn read<R: Read>(r: &mut R) -> Result<Self::Content, PacketReadError> {
        Ok(UpdateLightPacket {
            chunk_x: PacketDataType::read(r)?,
            chunk_z: PacketDataType::read(r)?,
            light: read_light_content(r)?,
        })
    }
}

fn write_light_content<W: Write>(light: LightPacket, w: &mut W) -> Result<(), PacketWriteError> {
    light.trust_edges.write(w)?;
    light.sky_light_mask.write(w)?;
    light.block_light_mask.write(w)?;
    light.empty_sky_light_mask.write(w)?;
    light.empty_block_light_mask.write(w)?;
    light.sky_light.write(w)?;
    light.block_light.write(w)?;
    Ok(())
}
fn read_light_content<R: Read>(reader: &mut R) -> Result<LightPacket, PacketReadError> {
    let trust_edges = bool::read(reader)?;
    let sky_light_mask = Vec::read(reader)?;
    let block_light_mask = Vec::read(reader)?;
    let empty_sky_light_mask = Vec::read(reader)?;
    let empty_block_light_mask = Vec::read(reader)?;
    let sky_light: Vec<u8> = Vec::read(reader)?;
    let block_light: Vec<u8> = Vec::read(reader)?;
    Ok(LightPacket {
        trust_edges,
        sky_light_mask,
        block_light_mask,
        empty_sky_light_mask,
        empty_block_light_mask,
        sky_light,
        block_light,
    })
}
