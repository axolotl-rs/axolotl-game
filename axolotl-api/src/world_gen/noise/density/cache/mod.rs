use std::sync::atomic::{AtomicU64, Ordering};

use all_in_cell::AllInCellCache;
use flat::FlatCache;
use once::OnceCache;
use two_d::TwoDCache;

use crate::world_gen::noise::density::groups::{define_group, define_group_def};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::BuildDefResult;
use crate::world_gen::noise::density::DensityContext;
use crate::world_gen::noise::density::DensityState;
use crate::world_gen::noise::density::FunctionArgument;
use crate::world_gen::noise::density::Game;
use crate::world_gen::noise::DensityLoader;
use crate::world_gen::noise::Noise;
use crate::NamespacedKey;

use super::DensityFunction;

pub mod all_in_cell;
pub mod flat;
pub mod once;
pub mod two_d;

#[derive(Debug)]
pub struct AtomicF64 {
    storage: AtomicU64,
}
impl AtomicF64 {
    pub fn new(value: f64) -> Self {
        let as_u64 = value.to_bits();
        Self {
            storage: AtomicU64::new(as_u64),
        }
    }
    pub fn store(&self, value: f64, ordering: Ordering) {
        let as_u64 = value.to_bits();
        self.storage.store(as_u64, ordering)
    }
    pub fn load(&self, ordering: Ordering) -> f64 {
        let as_u64 = self.storage.load(ordering);
        f64::from_bits(as_u64)
    }
}

define_group_def!(
    CacheGroupDef,
    AllInCellCache,
    AllInCellCache,
    FlatCache,
    FlatCache,
    OnceCache,
    OnceCache,
    TwoDCache,
    TwoDCache
);

define_group!(
    CacheFunctions,
    CacheGroupDef,
    AllInCellCache,
    AllInCellCache,
    "all_in_cell",
    FlatCache,
    FlatCache,
    "flat",
    OnceCache,
    OnceCache,
    "once",
    TwoDCache,
    TwoDCache,
    "two_d"
);
