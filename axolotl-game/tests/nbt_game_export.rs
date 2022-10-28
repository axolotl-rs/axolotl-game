use axolotl_api::data::GenericPacketVersion;
use axolotl_api::world_gen::biome::vanilla::{BiomePacket, DataPackBiome};
use axolotl_api::world_gen::dimension::Dimension;
use axolotl_game::chat::AxolotlChatType;
use axolotl_game::registry::SerializeRegistry;
use axolotl_game::GameConfig;
use axolotl_nbt::serde_impl;
use axolotl_nbt::value::Value;
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct TestStruct<'registry> {
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome:
        SerializeRegistry<'registry, GenericPacketVersion<'registry, BiomePacket<'registry>>>,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension: SerializeRegistry<'registry, GenericPacketVersion<'registry, Dimension>>,
    #[serde(rename = "minecraft:chat_type")]
    pub chat: SerializeRegistry<'registry, GenericPacketVersion<'registry, AxolotlChatType>>,
}
#[test]
pub fn test() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        axolotl_data: PathBuf::from(env!("AXOLOTL_DATA")),
    };
    let game = axolotl_game::AxolotlGame::load(config).unwrap();
    let biomes = SerializeRegistry {
        value: game.registries.biomes.as_packet_array(),
        registry_name: "minecraft:worldgen/biome",
    };
    let chat_types = SerializeRegistry {
        value: game.registries.chat_types.as_packet_array(),
        registry_name: "minecraft:chat_type",
    };

    let dimensions = SerializeRegistry {
        value: game.data_registries.dimensions.as_packet_array(),
        registry_name: "minecraft:dimension_type",
    };
    let test = TestStruct {
        chat: chat_types,
        dimension: dimensions,
        biome: biomes,
    };
    let file = PathBuf::new().join("registry-codec.json");
    serde_json::to_writer_pretty(File::create(file).unwrap(), &test).unwrap();

    let nbt_file = PathBuf::new().join("registry-codec.nbt");
    serde_impl::to_writer(&mut File::create(&nbt_file).unwrap(), &test).unwrap();

    let value: Value = serde_impl::from_reader_binary(&mut File::open(nbt_file).unwrap()).unwrap();
}
