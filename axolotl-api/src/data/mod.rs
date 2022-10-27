use crate::world_gen::biome::vanilla::{BiomePacket, VanillaPrecipitation};
use crate::world_gen::biome::Biome;
use axolotl_types::OwnedNameSpaceKey;
use serde::Serialize;

pub trait ForPacket {
    type PacketVersion<'p>: Serialize
    where
        Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: &'p OwnedNameSpaceKey,
    ) -> Self::PacketVersion<'p>;
}
#[derive(Debug, Serialize)]
pub struct PacketVersion<'p, T: Serialize + 'p> {
    pub id: usize,
    pub namespace: &'p OwnedNameSpaceKey,
    pub data: T,
}
impl<B: Biome<Precipitation = VanillaPrecipitation>> ForPacket for B {
    type PacketVersion<'p> = PacketVersion<'p, BiomePacket<'p>> where
    Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: &'p OwnedNameSpaceKey,
    ) -> Self::PacketVersion<'p> {
        PacketVersion {
            id,
            namespace,
            data: BiomePacket {
                downfall: self.get_downfall(),
                precipitation: self.get_precipitation(),
                effects: self.get_effects(),
                temperature: self.temperature(),
            },
        }
    }
}
