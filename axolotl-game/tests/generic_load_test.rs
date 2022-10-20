use axolotl_game::GameConfig;
use std::path::PathBuf;
#[test]
pub fn load_game() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        prismarine_data: PathBuf::from(env!("PRISMARINE_DATA")),
    };
    let game = axolotl_game::AxolotlGame::load(config).unwrap();
}
