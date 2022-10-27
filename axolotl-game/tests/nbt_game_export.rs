use axolotl_game::GameConfig;
use std::path::PathBuf;

#[test]
pub fn test() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        axolotl_data: PathBuf::from(env!("AXOLOTL_DATA")),
    };
    let game = axolotl_game::AxolotlGame::load(config).unwrap();
}
