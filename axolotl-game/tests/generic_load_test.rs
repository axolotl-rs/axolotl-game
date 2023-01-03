use std::path::PathBuf;
use std::sync::Arc;

use axolotl_api::world::{BlockPosition, World};
use axolotl_api::world_gen::chunk::ChunkPos;
use axolotl_api::world_gen::noise::ChunkGenerator;
use axolotl_game::world::chunk::placed_block::PlacedBlock;
use axolotl_game::world::chunk::AxolotlChunk;
use axolotl_game::world::generator::AxolotlGenerator;
use axolotl_game::GameConfig;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TestWorld {}
impl World for TestWorld {
    type Chunk = AxolotlChunk<Self>;
    type WorldBlock = PlacedBlock<Self>;
    type NoiseGenerator = AxolotlGenerator<Self>;

    fn get_name(&self) -> &str {
        todo!()
    }

    fn tick(&mut self) {
        todo!()
    }

    fn generator(&self) -> &Self::NoiseGenerator {
        todo!()
    }

    fn set_block(
        &self,
        location: BlockPosition,
        block: Self::WorldBlock,
        require_loaded: bool,
    ) -> bool {
        todo!()
    }

    fn set_blocks(
        &self,
        chunk_pos: ChunkPos,
        blocks: impl Iterator<Item = (BlockPosition, Self::WorldBlock)>,
    ) {
        todo!()
    }
}
#[test]
pub fn load_game() {
    simple_log::quick!();
    let data_dump = option_env!("DATA_DUMP").unwrap_or("data_dump");
    let axolotl_data = option_env!("AXOLOTL_DATA").unwrap_or("axolotl_data");
    let config = GameConfig {
        data_dump: PathBuf::from(data_dump),
        data_packs: vec![],
        axolotl_data: PathBuf::from(axolotl_data),
    };
    let game = axolotl_game::AxolotlGame::<TestWorld>::load(config)
        .map(Arc::new)
        .unwrap();

    println!("{:#?}", game);
}
