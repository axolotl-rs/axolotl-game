use std::fmt::Debug;
use std::sync::Arc;

use crate::world::World;

/// Generic Location Type
pub trait Location: Debug {
    /// get X coordinate
    fn get_x(&self) -> f64;
    /// get Y coordinate
    fn get_y(&self) -> f64;
    /// get Z coordinate
    fn get_z(&self) -> f64;
    /// get yaw
    fn get_yaw(&self) -> f64;
    /// get pitch
    fn get_pitch(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenericLocation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
}

impl Location for GenericLocation {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z
    }
    fn get_yaw(&self) -> f64 {
        self.yaw
    }
    fn get_pitch(&self) -> f64 {
        self.pitch
    }
}

impl From<(f64, f64, f64)> for GenericLocation {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self {
            x,
            y,
            z,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct WorldLocation<W: World> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
    pub world: Arc<W>,
}

impl<W: World> From<(GenericLocation, Arc<W>)> for WorldLocation<W> {
    fn from((location, world): (GenericLocation, Arc<W>)) -> Self {
        Self {
            x: location.x,
            y: location.y,
            z: location.z,
            yaw: location.yaw,
            pitch: location.pitch,
            world,
        }
    }
}

impl<W: World> Location for WorldLocation<W> {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z
    }
    fn get_yaw(&self) -> f64 {
        self.yaw
    }
    fn get_pitch(&self) -> f64 {
        self.pitch
    }
}
