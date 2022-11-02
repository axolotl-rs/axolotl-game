use log::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactArray {
    pub bits_per_block: usize,
    pub data: Vec<u64>,
    pub length: usize,
    pub values_per_u64: usize,
    pub mask: u64,
}
impl Into<Vec<u64>> for CompactArray {
    fn into(self) -> Vec<u64> {
        self.data
    }
}
impl CompactArray {
    pub fn new(bits_per_block: usize, length: usize) -> Self {
        let values_per_u64 = Self::calc_values_per_u64(bits_per_block);

        let data = vec![0; length / values_per_u64];
        let array = CompactArray {
            bits_per_block,
            data,
            length,
            values_per_u64,
            mask: Self::calc_mask(bits_per_block),
        };
        array
    }
    pub fn replace_inner(&mut self, data: Vec<u64>) {
        self.data = data;
    }
    #[inline(always)]
    pub fn calc_mask(bits_per_block: usize) -> u64 {
        (1 << bits_per_block as u64) - 1
    }
    #[inline(always)]
    pub fn calc_values_per_u64(bits_per_block: usize) -> usize {
        64 / bits_per_block
    }
    pub fn new_from_vec(bits_per_block: usize, data: Vec<u64>, length: usize) -> Self {
        let values_per_u64 = Self::calc_values_per_u64(bits_per_block);
        CompactArray {
            bits_per_block,
            length,
            data,
            values_per_u64,
            mask: Self::calc_mask(bits_per_block),
        }
    }
    pub fn get(&self, index: impl CompactArrayIndex) -> Option<u64> {
        let index = index.get();
        if index >= self.length {
            return None;
        }
        let (index, offset) = self.index_bit_value(index);
        Some((self.data[index] >> offset) & self.mask as u64)
    }
    pub fn set(&mut self, index: impl CompactArrayIndex, value: u64) {
        let index = index.get();
        let (index, offset) = self.index_bit_value(index);
        let re = &mut self.data[index];
        *re &= !(self.mask << offset);
        *re |= value << offset;
    }
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        Self::iter_vec(
            &self.data,
            self.values_per_u64,
            self.bits_per_block,
            self.mask,
        )
    }

    pub fn iter_vec(
        data: &Vec<u64>,
        values_per_u64: usize,
        bits_per_block: usize,
        mask: u64,
    ) -> impl Iterator<Item = u64> + '_ {
        data.iter()
            .flat_map(move |x| (0..values_per_u64).map(move |i| (x >> (i * bits_per_block)) & mask))
    }
    pub fn iter_vec_with_location(
        data: &Vec<u64>,
        values_per_u64: usize,
        bits_per_block: usize,
        mask: u64,
    ) -> impl Iterator<Item = (u64, u64)> + '_ {
        data.iter().enumerate().flat_map(move |(data_index, x)| {
            let index_location = data_index * values_per_u64;
            (0..values_per_u64).map(move |i| {
                let value = (x >> (i * bits_per_block)) & mask;
                let location = ((index_location + i) / bits_per_block) as u64;
                return (value, location);
            })
        })
    }
    #[inline]
    fn index_bit_value(&self, index: usize) -> (usize, usize) {
        let list_index = (index / self.values_per_u64) as usize;

        let bit_offset = (index % self.values_per_u64) * self.bits_per_block;
        (list_index, bit_offset)
    }
}

pub trait CompactArrayIndex {
    fn get(self) -> usize;
}
impl CompactArrayIndex for u64 {
    #[inline(always)]
    fn get(self) -> usize {
        self as usize
    }
}

impl CompactArrayIndex for usize {
    #[inline(always)]
    fn get(self) -> usize {
        self
    }
}
macro_rules! define_index_type {
    ($data_ty:ty) => {
        impl CompactArrayIndex for ($data_ty, $data_ty, $data_ty) {
            #[inline(always)]
            fn get(self) -> usize {
                let (x, y, z) = self;
                ((y << 8) | (z << 4) | x) as usize
            }
        }
    };
}
define_index_type!(u32);
define_index_type!(u64);
define_index_type!(usize);
define_index_type!(i32);
define_index_type!(i64);
#[cfg(test)]
pub mod tests {
    use crate::chunk::compact_array::CompactArrayIndex;

    #[test]
    pub fn number_tests() {
        println!("{:#066b}", (0u32, 1u32, 2u32).get())
    }
}
