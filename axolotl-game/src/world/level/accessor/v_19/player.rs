use crate::Error;
use ahash::AHashMap;
use axolotl_nbt::serde_impl;
use axolotl_world::entity::player::PlayerData;
use parking_lot::RwLock;
use std::fs::File;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug)]
pub struct Minecraft19PlayerAccess {
    // Key is Player UUID and Value is a hash of the world's name
    pub loaded_players: RwLock<AHashMap<Uuid, u64>>,
    pub player_folder: PathBuf,
}
impl Minecraft19PlayerAccess {
    pub fn new(player_folder: PathBuf) -> Self {
        Self {
            loaded_players: RwLock::new(AHashMap::new()),
            player_folder,
        }
    }
    pub fn save_player(&self, uuid: Uuid, player: &PlayerData) -> Result<(), Error> {
        let mut guard = self.loaded_players.write();
        guard.remove(&uuid);
        let mut file = File::create(
            self.player_folder
                .join(format!("{}.dat", uuid.hyphenated())),
        )?;
        serde_impl::to_writer(&mut file, player)?;
        Ok(())
    }
    pub fn get_player(
        &self,
        uuid: Uuid,
        source_world: u64,
    ) -> Result<Option<PlayerData>, crate::Error> {
        let loaded_players = self.loaded_players.read();
        if loaded_players.contains_key(&uuid) {
            return Ok(None);
        }
        drop(loaded_players);
        let mut loaded_players = self.loaded_players.write();
        if loaded_players.contains_key(&uuid) {
            return Ok(None);
        }
        loaded_players.insert(uuid, source_world);
        drop(loaded_players);
        let player_data = self
            .player_folder
            .join(format!("{}.dat", uuid.hyphenated()));
        if !player_data.exists() {
            return Ok(Some(PlayerData::default()));
        }
        let data: PlayerData = serde_impl::from_reader_binary(File::open(player_data)?)?;
        Ok(Some(data))
    }
}
