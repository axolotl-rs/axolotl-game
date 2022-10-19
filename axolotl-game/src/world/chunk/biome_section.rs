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
}
