use crate::world::block::MinecraftBlock;
use crate::world::chunk::consts::{
    BITS_PER_BLOCK, SECTION_SIZE, SECTION_X_SIZE, SECTION_Y_SIZE, SECTION_Z_SIZE,
};
use crate::world::chunk::placed_block::PlacedBlock;
use crate::world::chunk::section::{InvalidChunkSection, SectionPosIndex};

use crate::AxolotlGame;
use axolotl_api::NameSpaceRef;
use axolotl_world::chunk::compact_array::CompactArray;
use axolotl_world::chunk::BlockStates;
use log::warn;
use std::mem;
use std::mem::discriminant;

/// Returns Err(()) if block is outside of the range
#[derive(Debug, Clone, Default)]
pub enum AxolotlBlockSection<'game> {
    /// All Air
    #[default]
    Empty,
    /// All of one block type.  Note: Could be air.
    SingleBlock(PlacedBlock<'game>),
    Full {
        blocks: CompactArray,
        /// Will be Empty if is just air
        block_palette: Vec<PlacedBlock<'game>>,
    },
}
impl Into<BlockStates> for AxolotlBlockSection<'_> {
    fn into(self) -> BlockStates {
        match self {
            AxolotlBlockSection::Empty => BlockStates {
                data: None,
                palette: vec![],
            },
            AxolotlBlockSection::SingleBlock(block) => BlockStates {
                data: None,
                palette: vec![block.into()],
            },
            AxolotlBlockSection::Full {
                blocks,
                block_palette,
            } => {
                let palette = block_palette
                    .into_iter()
                    .map(|b| b.into())
                    .collect::<Vec<_>>();
                BlockStates {
                    data: Some(blocks.into()),
                    palette,
                }
            }
        }
    }
}
impl PartialEq for AxolotlBlockSection<'_> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
impl<'game, Pos: Into<SectionPosIndex>, Block: Into<PlacedBlock<'game>>, Iter> From<Iter>
    for AxolotlBlockSection<'game>
where
    Iter: IntoIterator<Item = (Pos, Block)>,
{
    fn from(iter: Iter) -> Self {
        let mut blocks = CompactArray::new(BITS_PER_BLOCK, SECTION_SIZE);
        let mut block_palette = Vec::new();

        for (pos, block) in iter {
            let pos = pos.into();
            let block = block.into();
            if let Some(v) = block_palette.iter().position(|b| b == &block) {
                blocks.set(pos, v as u64);
            } else {
                let index = block_palette.len();
                block_palette.push(block);
                blocks.set(pos, index as u64);
            }
        }
        if block_palette.len() == 1 {
            let block = block_palette.pop().unwrap();
            AxolotlBlockSection::SingleBlock(block)
        } else {
            AxolotlBlockSection::Full {
                blocks,
                block_palette,
            }
        }
    }
}

impl<'game> AxolotlBlockSection<'game> {
    pub fn set_block(&mut self, pos: impl Into<SectionPosIndex>, block: PlacedBlock<'game>) {
        let pos = pos.into();
        match self {
            AxolotlBlockSection::Empty => {
                if block.is_air() {
                    return;
                }
                *self = AxolotlBlockSection::SingleBlock(block);
            }
            AxolotlBlockSection::SingleBlock(placed) => {
                let (loc_x, loc_y, loc_z) = <SectionPosIndex as Into<(u64, u64, u64)>>::into(pos);
                let mut compact = CompactArray::new(BITS_PER_BLOCK, SECTION_SIZE);
                for x in 0..SECTION_X_SIZE as u64 {
                    for y in 0..SECTION_Y_SIZE as u64 {
                        for z in 0..SECTION_Z_SIZE as u64 {
                            if x == loc_x && y == loc_y && z == loc_z {
                                compact.set(SectionPosIndex::from((x, y, z)), 1);
                            } else {
                                compact.set(SectionPosIndex::from((x, y, z)), 0);
                            }
                        }
                    }
                }
                let placed_blocks = vec![mem::take(placed), block];

                *self = AxolotlBlockSection::Full {
                    blocks: compact,
                    block_palette: placed_blocks,
                };
            }
            AxolotlBlockSection::Full {
                blocks,
                block_palette,
            } => {
                if let Some(v) = block_palette.iter().position(|b| b == &block) {
                    blocks.set(pos, v as u64);
                } else {
                    let index = block_palette.len();
                    block_palette.push(block);
                    blocks.set(pos, index as u64);
                }
            }
        }
    }
    pub fn get_block(&self, pos: impl Into<SectionPosIndex>) -> Option<&PlacedBlock<'game>> {
        match self {
            AxolotlBlockSection::Empty => None,
            AxolotlBlockSection::SingleBlock(placed) => Some(placed),
            AxolotlBlockSection::Full {
                blocks,
                block_palette,
            } => {
                let result = pos.into();
                let option = blocks.get(result);
                if let Some(index) = option {
                    let index = index as usize;
                    block_palette.get(index)
                } else {
                    None
                }
            }
        }
    }

    pub fn load(
        &mut self,
        game: &'game AxolotlGame,
        section: &mut BlockStates,
    ) -> Result<(), InvalidChunkSection> {
        if section.data.is_none() {
            return if let Some(block) = section.palette.pop() {
                let mc_block = game.get_block(&block.name);
                if let Some(block) = mc_block {
                    *self = AxolotlBlockSection::SingleBlock(PlacedBlock::from(block));
                    Ok(())
                } else {
                    Err(InvalidChunkSection::InvalidNamespaceKey(block.name))
                }
            } else {
                if self == &AxolotlBlockSection::Empty {
                    *self = AxolotlBlockSection::Empty;
                }
                Ok(())
            };
        } else if let Some(data) = section.data.take() {
            match self {
                AxolotlBlockSection::Full {
                    blocks,
                    block_palette,
                } => {
                    blocks.replace_inner(data);
                    if block_palette.len() > section.palette.len() {
                        block_palette.truncate(section.palette.len());
                    }
                    for (i, block) in section.palette.iter().enumerate() {
                        let mc_block = game.get_block(&block.name);
                        block_palette[i] = if let Some(block) = mc_block {
                            PlacedBlock::from(block)
                        } else {
                            warn!("Invalid block: {}", block.name);
                            PlacedBlock::from(
                                game.get_block(NameSpaceRef::new("minecraft", "air"))
                                    .unwrap(),
                            )
                        };
                    }
                }
                v => {
                    let mut placed_blocks = Vec::with_capacity(section.palette.len());
                    for block in section.palette.iter() {
                        let mc_block = game.get_block(&block.name);
                        if let Some(block) = mc_block {
                            placed_blocks.push(PlacedBlock::from(block));
                        } else {
                            warn!("Invalid block: {}", block.name);
                        }
                    }

                    *v = AxolotlBlockSection::Full {
                        blocks: CompactArray::new_from_vec(BITS_PER_BLOCK, data, SECTION_SIZE),
                        block_palette: placed_blocks,
                    };
                }
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AxolotlBlockSection::Empty => true,
            AxolotlBlockSection::SingleBlock(v) => {
                if let MinecraftBlock::Air = v.block {
                    true
                } else {
                    false
                }
            }
            AxolotlBlockSection::Full { .. } => false,
        }
    }
}
