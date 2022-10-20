use ahash::AHashMap;
#[derive(Debug, Clone)]
pub struct BlockMaterial {
    // Ex "wool", "leaves;mineable/axe;mineable/hoe"
    pub applies_to: Vec<String>,
    pub values: AHashMap<u64, f64>,
    #[cfg(feature = "custom_blocks")]
    pub custom_values: AHashMap<String, f64>,
}
