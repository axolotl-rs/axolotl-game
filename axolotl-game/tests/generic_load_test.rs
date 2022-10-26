use axolotl_game::GameConfig;
use std::path::PathBuf;

#[test]
pub fn load_game() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        axolotl_data: PathBuf::from(env!("AXOLOTL_DATA")),
    };
    let _game = axolotl_game::AxolotlGame::load(config).unwrap();
}
