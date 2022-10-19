use bytemuck::{Contiguous, Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::ops::{Add, Sub};

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
impl<N: From<i32>> Into<(N, N)> for ChunkPos {
    fn into(self) -> (N, N) {
        (self.0.into(), self.1.into())
    }
}
impl Into<(i32, i32)> for &'_ ChunkPos {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}
impl Into<i64> for ChunkPos {
    fn into(self) -> i64 {
        into_condensed_location(self.0 as i64, self.1 as i64) as i64
    }
}
impl Into<u64> for ChunkPos {
    fn into(self) -> u64 {
        into_condensed_location(self.0 as i64, self.1 as i64)
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
