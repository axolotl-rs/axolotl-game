use crate::blocks::generic_block::GenericBlock;
use crate::blocks::raw_state::RawState;
use crate::blocks::{InnerMinecraftBlock, MinecraftBlock};
use axolotl_api::game::{Game, Registry};

use axolotl_api::{NamespacedId, NumericId};
use log::{debug, warn};
use std::collections::HashMap;

use crate::blocks::v19::bed::BedBlock;

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

pub fn load_blocks<G: Game>(
    minecraft_data: PathBuf,
    data_dump: PathBuf,
    materials: &ahash::HashMap<String, Arc<Material>>,
    register: &mut impl Registry<MinecraftBlock<G>>,
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

    for block in blocks {
        // Turn block into generic block
        if !block.tags.is_empty() {
            let tag = &block.tags[0];
            match tag.name.as_str() {
                "BedBlock" => {
                    let block = BedBlock::new(block, materials, &mut states);
                    register.register_with_id(
                        &format!("minecraft:{}", block.key),
                        block.id(),
                        Arc::new(InnerMinecraftBlock::DynamicBlock(Box::new(block))),
                    );
                    continue;
                }
                "AirBlock" => {
                    let block = InnerMinecraftBlock::Air {
                        id: block.id,
                        key: block.name,
                    };
                    register.register_with_id(
                        &format!("minecraft:{}", block.key()),
                        block.id(),
                        Arc::new(block),
                    );
                    continue;
                }
                _ => {
                    warn!("Unknown Top Level Tag: {}. Registering as a Generic could mean missing game features", tag.name);
                }
            }
        }
        // Default to generic block
        let block = GenericBlock::new(block, materials, &mut states);
        register.register_with_id(
            &format!("minecraft:{}", block.0.key),
            block.id(),
            Arc::new(InnerMinecraftBlock::GenericBlock(block)),
        );
    }
    Ok(())
}
