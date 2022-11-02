use crate::world::chunk::section::SectionPosIndex;
use axolotl_api::world::BlockPosition;
use axolotl_api::OwnedNameSpaceKey;
use axolotl_world::chunk::compact_array::CompactArray;
use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum AxolotlBiomeSection {
    /// One type of biome lives here
    SingleBiome(OwnedNameSpaceKey),
    Full {
        biome_palette: Vec<OwnedNameSpaceKey>,
        biomes: CompactArray,
    },
}

impl PartialEq for AxolotlBiomeSection {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
impl AxolotlBiomeSection {
    pub fn new(namespace_key: impl Into<OwnedNameSpaceKey>) -> Self {
        AxolotlBiomeSection::SingleBiome(namespace_key.into())
    }

    pub fn set_biome(&mut self, pos: impl Into<SectionPosIndex>, value: OwnedNameSpaceKey) {
        // TODO implement full BiomeSection
        match self {
            AxolotlBiomeSection::SingleBiome(v) => {
                *v = value;
            }
            AxolotlBiomeSection::Full {
                biome_palette,
                biomes,
            } => {
                todo!()
            }
        }
    }
}
