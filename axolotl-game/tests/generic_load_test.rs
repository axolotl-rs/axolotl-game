use axolotl_api::OwnedNameSpaceKey;
use axolotl_game::world::generator::{AxolotlGenerator, ChunkSettings};
use axolotl_game::world::level::accessor::v_19::player::Minecraft19PlayerAccess;
use axolotl_game::world::level::configs::WorldConfig;
use axolotl_game::world::level::flat::{FlatSettings, Layer};
use axolotl_game::world::{AxolotlWorld, ChunkUpdate};
use axolotl_game::{AxolotlGame, GameConfig};
use log::info;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[test]
pub fn load_game() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        axolotl_data: PathBuf::from(env!("AXOLOTL_DATA")),
    };
    let game = axolotl_game::AxolotlGame::load(config)
        .map(Arc::new)
        .unwrap();

    let world = Path::new("world");
    if world.exists() {
        std::fs::remove_dir_all(world).unwrap();
    }
    std::fs::create_dir(world).unwrap();

    let world = world.canonicalize().unwrap();
    let player = world.join("player");
    if player.exists() {
        std::fs::remove_dir_all(&player).unwrap();
    }
    std::fs::create_dir(&player).unwrap();

    info!("Attempting to create a world at {:?}", world);

    let world_load = AxolotlWorld::create(
        game.clone(),
        Uuid::new_v4(),
        "world".to_string(),
        8,
        8,
        world,
        ChunkSettings::Flat {
            settings: FlatSettings {
                biome: "minecraft:plains".to_string(),
                features: false,
                lakes: false,
                layers: vec![
                    Layer {
                        block: "minecraft:bedrock".to_string(),
                        height: 1,
                    },
                    Layer {
                        block: "minecraft:dirt".to_string(),
                        height: 2,
                    },
                    Layer {
                        block: "minecraft:oak_planks".to_string(),
                        height: 1,
                    },
                ],
                structure_overrides: vec![],
            },
        },
        Arc::new(Minecraft19PlayerAccess::new(player)),
        0,
        OwnedNameSpaceKey::new("minecraft".to_string(), "overworld".to_string()),
    )
    .expect("Failed to create world");
    let axolotl_world = world_load.world;
    for x in 0..=32 {
        for z in 0..=32 {
            axolotl_world.chunk_map.queue.push(ChunkUpdate::Load {
                x,
                z,
                set_block: None,
            });
        }
    }
    info!("Creating Chunks");
    axolotl_world.chunk_map.handle_updates();
    info!("Done Creating Chunks");
    info!("Saving Chunks");
    axolotl_world.chunk_map.save_all();
    info!("Done Saving Chunks");
    axolotl_world.chunk_map.accessor.force_close_all();
}
