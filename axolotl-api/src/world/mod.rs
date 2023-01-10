use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use bytemuck::{Pod, Zeroable};
use serde::de::MapAccess;
use serde::{Deserialize, Serialize};

pub use location::GenericLocation;
pub use location::Location;
pub use location::WorldLocation;

use crate::item::block::{Block, BlockState};
use crate::world_gen::chunk::ChunkPos;

mod location;

pub struct WorldGenerator {
    pub seed: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Zeroable)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i16,
    pub z: i64,
}
impl BlockPosition {
    pub fn new(x: i64, y: i16, z: i64) -> Self {
        Self { x, y, z }
    }
    pub fn absolute(&self) -> Self {
        Self {
            x: self.x * 16,
            y: self.y,
            z: self.z * 16,
        }
    }
    pub fn absolute_ref(&mut self) {
        self.x *= 16;
        self.z *= 16;
    }
    pub fn make_relative_ref(&mut self) {
        self.x %= 16;
        self.z %= 16;
    }
    #[inline(always)]
    pub fn section(&mut self) -> usize {
        let section_index = ((self.y as usize) / 16);
        self.y %= 16;
        section_index
    }
    /// Returns the chunk position of the chunk this block is in
    /// Makes the x.y relative to the chunk
    #[inline(always)]
    pub fn chunk(&mut self) -> ChunkPos {
        let x = (self.x / 16);
        let z = (self.z / 16);
        self.x %= 16;
        self.z %= 16;
        ChunkPos::new(x as i32, z as i32)
    }
}
impl<L: Location> From<L> for BlockPosition {
    fn from(l: L) -> Self {
        Self {
            x: l.get_x() as i64,
            y: l.get_y() as i16,
            z: l.get_z() as i64,
        }
    }
}

impl From<(i64, i16, i64)> for BlockPosition {
    fn from((x, y, z): (i64, i16, i64)) -> Self {
        Self { x, y, z }
    }
}

pub trait World: Send + Sync + Hash + Debug + PartialEq {
    type Chunk;
    type WorldBlock;
    type NoiseGenerator: crate::world_gen::noise::ChunkGenerator<Chunk = Self::Chunk>;
    fn get_name(&self) -> &str;

    fn tick(&mut self);

    fn generator(&self) -> &Self::NoiseGenerator;

    fn set_block(
        &self,
        location: BlockPosition,
        block: Self::WorldBlock,
        require_loaded: bool,
    ) -> bool;
    ///
    /// Rules for the group set chunk
    /// 1. They must all be in the same chunk
    /// 2. The chunk must be loaded
    /// 3. BlockPos must already be relative to the chunk
    fn set_blocks(
        &self,
        chunk_pos: ChunkPos,
        blocks: impl Iterator<Item = (BlockPosition, Self::WorldBlock)>,
    );
}
/// A WorldLocationID. This will Deserialize from either a string or a map
///
/// # Examples
/// {"group": "vanilla", "world": "overworld"}
/// "vanilla/overworld"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct WorldLocationID {
    pub group: String,
    pub world: String,
}
impl WorldLocationID {
    pub fn new(group: String, world: String) -> Self {
        Self { group, world }
    }
}
impl FromStr for WorldLocationID {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let group = split.next().ok_or("No group")?.to_string();
        let world = split.next().ok_or("No world")?.to_string();
        Ok(Self { group, world })
    }
}
impl Display for WorldLocationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.world)
    }
}
impl Serialize for WorldLocationID {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
struct WorldLocationIDVisitor;
impl<'de> serde::de::Visitor<'de> for WorldLocationIDVisitor {
    type Value = WorldLocationID;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string in the format of `group/world`")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse().map_err(serde::de::Error::custom)
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        v.parse().map_err(serde::de::Error::custom)
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map = map;
        let mut group = None;
        let mut world = None;
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "group" => group = Some(value),
                "world" => world = Some(value),
                _ => return Err(serde::de::Error::custom(format!("Unknown key `{}`", key))),
            }
        }
        Ok(WorldLocationID {
            group: group.ok_or_else(|| serde::de::Error::custom("Missing `group` key"))?,
            world: world.ok_or_else(|| serde::de::Error::custom("Missing `world` key"))?,
        })
    }
}
impl<'de> Deserialize<'de> for WorldLocationID {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(WorldLocationIDVisitor)
    }
}
