use auto_impl::auto_impl;
use std::panic::Location;
use uuid::Uuid;

use crate::world::{World, WorldLocation, WorldLocationID};

#[auto_impl(Arc, &, Box)]
pub trait GenericPlayer {
    fn get_uuid(&self) -> &Uuid;

    fn get_saved_name(&self) -> &str;
}
#[auto_impl(Arc, &, Box)]
pub trait Player: GenericPlayer {
    fn get_name(&self) -> &str;

    fn teleport(&self, location: Location);

    fn change_world(&self, world: &WorldLocationID, location: Location);
}
