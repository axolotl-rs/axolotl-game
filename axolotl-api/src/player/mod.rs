use uuid::Uuid;

use crate::world::{World, WorldLocation};

pub trait GenericPlayer {
    fn get_uuid(&self) -> &Uuid;

    fn get_saved_name(&self) -> &str;
}

pub trait Player: GenericPlayer {
    fn get_name(&self) -> &str;

    fn teleport<W: World, Location>(&self, new_location: Location) -> bool
    where
        Location: Into<WorldLocation<W>>;
}
