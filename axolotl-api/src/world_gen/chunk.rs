use std::hash::Hash;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPos(i64);
impl PartialEq<(i64, i64)> for ChunkPos {
    fn eq(&self, (cx, cz): &(i64, i64)) -> bool {
        let (x, z) = self.as_xz();
        x == *cx && z == *cz
    }
}
impl Hash for ChunkPos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Add for ChunkPos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, z) = self.as_xz();
        let (x2, z2) = rhs.as_xz();
        Self::new(x + x2, z + z2)
    }
}
impl Sub for ChunkPos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (x, z) = self.as_xz();
        let (x2, z2) = rhs.as_xz();
        Self::new(x - x2, z - z2)
    }
}
impl Add<(i64, i64)> for ChunkPos {
    type Output = Self;

    fn add(self, (x2, z2): (i64, i64)) -> Self::Output {
        let (x, y) = self.as_xz();
        Self::new(x + x2, y + z2)
    }
}
impl Sub<(i64, i64)> for ChunkPos {
    type Output = Self;

    fn sub(self, (x2, z2): (i64, i64)) -> Self::Output {
        let (x, y) = self.as_xz();
        Self::new(x - x2, y - z2)
    }
}
impl ChunkPos {
    pub fn new(x: i64, z: i64) -> Self {
        Self(x_z_to_chunk_i64(x, z))
    }
    pub fn x(&self) -> i64 {
        (self.0 & 0xFFFF_FFFF)
    }
    pub fn z(&self) -> i64 {
        (self.0 >> 32)
    }
    pub fn as_xz(&self) -> (i64, i64) {
        (self.x(), self.z())
    }
}
pub fn x_z_to_chunk_i64(x: i64, z: i64) -> i64 {
    ((x as i64 & 4294967295) | (z as i64 & 4294967295) << 32)
}
#[test]
pub fn test() {
    println!("{:#066b}", x_z_to_chunk_i64(0, 0));
    println!("{:#066b}", x_z_to_chunk_i64(1, 0));
    println!("{:#066b}", x_z_to_chunk_i64(0, 1));

    let pos = ChunkPos::new(32, 64);
    println!("{:#066b}", pos.0);
    println!("X: {}", pos.x());
    println!("Z: {}", pos.z());
}
