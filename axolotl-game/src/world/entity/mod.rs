pub mod player;
pub mod properties;

use crate::world::AxolotlWorld;

#[derive(Debug)]
pub enum MinecraftEntity {}

impl MinecraftEntity {
    pub fn tick(&self, _world: &AxolotlWorld) {}
}
