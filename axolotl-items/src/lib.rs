use crate::blocks::generic_block::GenericBlock;
use crate::blocks::raw_state::RawState;
use crate::blocks::MinecraftBlock;
use axolotl_api::game::Registry;
use axolotl_api::item::block::Block;
use axolotl_api::item::Item;
use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

pub mod blocks;
pub mod materials;
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
pub fn load_blocks(
    minecraft_data: PathBuf,
    data_dump: PathBuf,
    register: &mut impl Registry<MinecraftBlock>,
) -> Result<(), Error> {
    // Load states from Minecraft Data Dump
    debug!("Loading block states");
    let blocks_json = data_dump.join("reports").join("blocks.json");
    let mut states: HashMap<String, RawState> =
        serde_json::from_reader(std::fs::File::open(blocks_json)?)?;
    // Load Blocks from Minecraft Data
    let data = minecraft_data
        .join("data")
        .join("pc")
        .join("1.19")
        .join("blocks.json");
    debug!("Loading block data");

    let blocks: Vec<minecraft_data_rs::models::block::Block> =
        serde_json::from_reader(std::fs::File::open(data).unwrap()).unwrap();
    // Register Air
    register.register_with_id("minecraft:air", 0, MinecraftBlock::Air);
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
            let block = GenericBlock::new(block, &mut states);
            register.register_with_id(
                &block.get_namespace().to_string(),
                block.id(),
                MinecraftBlock::GenericBlock(block),
            );
        }
    }
    Ok(())
}
