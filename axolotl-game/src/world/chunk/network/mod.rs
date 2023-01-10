use crate::world::chunk::sections::biome_section::AxolotlBiomeSection;
use crate::world::chunk::sections::blocks_section::AxolotlBlockSection;
use crate::world::chunk::AxolotlChunk;
use axolotl_api::world::World;
use minecraft_protocol::data::var_int::{VarInt, ZERO};
use minecraft_protocol::data::PacketDataType;
use minecraft_protocol::packets::play::client::chunk::GetVanillaId;
use minecraft_protocol::PacketWriteError;
use std::io::Write;

pub trait NetworkChunk<W: World> {
    fn write_chunk<Writer: Write>(
        chunk: &AxolotlChunk<W>,
        w: &mut Writer,
    ) -> Result<(), PacketWriteError>;

    fn write_block_section(
        section: &AxolotlBlockSection<W>,
        w: &mut impl Write,
    ) -> Result<(), PacketWriteError>;

    fn write_biome_section(
        section: &AxolotlBiomeSection,
        w: &mut impl Write,
    ) -> Result<(), PacketWriteError>;
}

pub struct NetworkChunk1_19<W: World>(std::marker::PhantomData<W>);
impl<W: World> NetworkChunk<W> for NetworkChunk1_19<W> {
    fn write_chunk<Writer: Write>(
        chunk: &AxolotlChunk<W>,
        w: &mut Writer,
    ) -> Result<(), PacketWriteError> {
        let air: i16 = chunk
            .sections
            .as_ref()
            .iter()
            .map(|x| x.blocks.count_air())
            .sum();
        air.write(w)?;
        for section in chunk.sections.as_ref().iter() {
            Self::write_block_section(&section.blocks, w)?;
        }
        for section in chunk.sections.as_ref().iter() {
            Self::write_biome_section(&section.biomes, w)?;
        }

        Ok(())
    }

    fn write_block_section(
        section: &AxolotlBlockSection<W>,
        w: &mut impl Write,
    ) -> Result<(), PacketWriteError> {
        match section {
            AxolotlBlockSection::Empty => {
                0u8.write(w)?;
                w.write(&ZERO)?;
                w.write(&ZERO)?;
            }
            AxolotlBlockSection::SingleBlock(block) => {
                0u8.write(w)?;
                VarInt(block.get_vanilla_id()).write(w)?;
                w.write(&ZERO)?;
            }
            AxolotlBlockSection::Full {
                blocks,
                block_palette,
            } => {
                0u8.write(w)?;
                VarInt(1).write(w)?;
                w.write(&ZERO)?;
            }
        }
        Ok(())
    }

    fn write_biome_section(
        section: &AxolotlBiomeSection,
        w: &mut impl Write,
    ) -> Result<(), PacketWriteError> {
        // TODO: Implement this
        0u8.write(w)?;
        w.write(&ZERO)?;
        w.write(&ZERO)?;
        Ok(())
    }
}
