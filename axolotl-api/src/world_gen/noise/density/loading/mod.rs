use serde_json::Value;

use crate::{NamespacedKey, OwnedNameSpaceKey};
use crate::world_gen::noise::{BiomeSource, NoiseSetting};

pub trait DensityLoader {
    type BiomeSource: BiomeSource;

    fn prep_for_load(&self, value: Value) -> UnloadedFunction;

    fn register_top_level(&mut self, key: OwnedNameSpaceKey, value: UnloadedFunction);

    fn get_settings(&self, name: impl NamespacedKey) -> &NoiseSetting;

    fn get_biome_source(&self, name: impl NamespacedKey) -> &Self::BiomeSource;
}

#[derive(Debug, Clone)]
pub enum UnloadedFunction {
    Function(Value),
    Reference(OwnedNameSpaceKey),
    Constant(f64),
}
