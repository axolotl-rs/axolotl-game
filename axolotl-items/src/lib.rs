use crate::blocks::generic_block::GenericBlock;
use crate::blocks::raw_state::RawState;
use crate::blocks::{InnerMinecraftBlock, MinecraftBlock};
use axolotl_api::game::Registry;

use axolotl_api::NumericId;
use log::debug;
use std::collections::HashMap;

use ahash::HashMapExt;
use axolotl_data_rs::blocks::{Block as RawBlock, Material};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

pub mod blocks;
pub mod items;
pub mod materials;
#[test]
pub fn test() {}
const HARD_CODED_BLOCKS: &str = include_str!("blocks/hard_coded/hard_coded.json");
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to load block file")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse block {0}")]
    Json(#[from] serde_json::Error),
}
#[test]
pub fn compile_test() {}
pub fn load_materials(
    minecraft_data: PathBuf,
) -> Result<ahash::HashMap<String, Arc<Material>>, Error> {
    let data = minecraft_data.join("materials.json");
    debug!("Loading block data");

    let blocks: Vec<Material> =
        serde_json::from_reader(std::fs::File::open(data).unwrap()).unwrap();

    let mut materials = ahash::HashMap::new();
    for material in blocks {
        materials.insert(material.name.clone(), Arc::new(material));
    }
    Ok(materials)
}

pub fn load_blocks(
    minecraft_data: PathBuf,
    data_dump: PathBuf,
    materials: &ahash::HashMap<String, Arc<Material>>,
    register: &mut impl Registry<MinecraftBlock>,
) -> Result<(), Error> {
    // Load states from Minecraft Data Dump
    debug!("Loading block states");
    let blocks_json = data_dump.join("reports").join("blocks.json");
    let mut states: HashMap<String, RawState> =
        serde_json::from_reader(std::fs::File::open(blocks_json)?)?;
    // Load Blocks from Minecraft Data
    let data = minecraft_data.join("blocks.json");
    debug!("Loading block data");

    let blocks: Vec<RawBlock> =
        serde_json::from_reader(std::fs::File::open(data).unwrap()).unwrap();
    // Register Air
    register.register_with_id("minecraft:air", 0, Arc::new(InnerMinecraftBlock::Air));
    // Loop through all blocks
    for block in blocks {
        // Skip Air
        if block.id == 0 {
            continue;
        }
        // Check if block is hard coded
        if let Some(v) = blocks::hard_coded::register_hard_coded(block.id) {
            register.register_with_id(&block.name, block.id as usize, v);
        } else {
            // Turn block into generic block
            let block = GenericBlock::new(block, &materials, &mut states);
            register.register_with_id(
                &format!("minecraft:{}", block.0.key),
                block.id(),
                Arc::new(InnerMinecraftBlock::GenericBlock(block)),
            );
        }
    }
    Ok(())
}
