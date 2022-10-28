use crate::chat::ChatType;
use crate::world_gen::biome::vanilla::{BiomePacket, VanillaPrecipitation};
use crate::world_gen::biome::Biome;
use crate::world_gen::dimension::Dimension;
use axolotl_types::{NameSpaceKey, OwnedNameSpaceKey};
use serde::{Serialize, Serializer};
use std::borrow::Cow;
pub trait PacketVersion: Serialize {
    fn id(&self) -> &usize;
}

pub trait ForPacket {
    type PacketVersion<'p>: PacketVersion
    where
        Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: impl Into<NameSpaceKey<'p>>,
    ) -> Self::PacketVersion<'p>;
}

#[derive(Debug, Serialize, Clone)]
pub struct GenericPacketVersion<'p, T: Serialize + Clone + 'p> {
    pub id: usize,
    pub name: NameSpaceKey<'p>,
    pub element: Cow<'p, T>,
}

impl<T: Serialize + Clone> PacketVersion for GenericPacketVersion<'_, T> {
    fn id(&self) -> &usize {
        &self.id
    }
}
impl<B: Biome<Precipitation = VanillaPrecipitation>> ForPacket for B {
    type PacketVersion<'p>
    = GenericPacketVersion<'p, BiomePacket<'p>>
    where
    Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: impl Into<NameSpaceKey<'p>>,
    ) -> Self::PacketVersion<'p> {
        GenericPacketVersion {
            id,
            name: namespace.into(),
            data: Cow::Owned(BiomePacket {
                downfall: self.get_downfall(),
                precipitation: self.get_precipitation(),
                effects: self.get_effects(),
                temperature: self.temperature(),
            }),
        }
    }
}
impl ForPacket for Dimension {
    type PacketVersion<'p>
    = GenericPacketVersion<'p, Self>    where
    Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: impl Into<NameSpaceKey<'p>>,
    ) -> Self::PacketVersion<'p> {
        GenericPacketVersion {
            id,
            name: namespace.into(),
            data: Cow::Borrowed(self),
        }
    }
}
