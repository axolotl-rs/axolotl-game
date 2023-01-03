use std::hash::Hash;

use bytemuck::{Pod, Zeroable};

#[inline(always)]
pub fn into_condensed_location(x: i64, z: i64) -> u64 {
    ((x as u64 & 4294967295) | (z as u64 & 4294967295) << 32)
}
#[inline(always)]
pub fn into_condensed_location_i32(x: i32, z: i32) -> u64 {
    ((x as u64 & 4294967295) | (z as u64 & 4294967295) << 32)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Pod, Zeroable)]
#[repr(C)]
pub struct ChunkPos(pub i32, pub i32);
impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self(x, z)
    }
    #[inline(always)]
    pub fn x(&self) -> i32 {
        self.0
    }
    #[inline(always)]
    pub fn z(&self) -> i32 {
        self.1
    }
}
impl<N: From<i32>> From<ChunkPos> for (N, N) {
    fn from(val: ChunkPos) -> Self {
        (val.0.into(), val.1.into())
    }
}
impl From<&'_ ChunkPos> for (i32, i32) {
    fn from(val: &'_ ChunkPos) -> Self {
        (val.0, val.1)
    }
}
impl From<ChunkPos> for i64 {
    fn from(val: ChunkPos) -> Self {
        into_condensed_location(val.0 as i64, val.1 as i64) as i64
    }
}
impl From<ChunkPos> for u64 {
    fn from(val: ChunkPos) -> Self {
        into_condensed_location(val.0 as i64, val.1 as i64)
    }
}
impl From<i64> for ChunkPos {
    fn from(value: i64) -> Self {
        Self(
            (value & 4294967295) as i32,
            ((value >> 32) & 4294967295) as i32,
        )
    }
}

#[test]
pub fn test() {
    println!("{:#066b}", into_condensed_location(0, 0));
    println!("{:#066b}", into_condensed_location(1, 0));
    println!("{:#066b}", into_condensed_location(0, 1));

    let pos = ChunkPos::new(32, 64);
    println!("{:#066b}", pos.0);
    println!("X: {}", pos.x());
    println!("Z: {}", pos.z());
}
