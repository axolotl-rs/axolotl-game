use axolotl_world::world::axolotl::level_dat::AxolotlLevelDat;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn create_test_world() {
    let world_folder = PathBuf::new().join("test_data").join("world");
    if world_folder.exists() {
        std::fs::remove_dir_all(&world_folder).unwrap();
    }
    create_dir_all(&world_folder).unwrap();

    let _level_dat = AxolotlLevelDat {
        ..AxolotlLevelDat::default()
    };
}
