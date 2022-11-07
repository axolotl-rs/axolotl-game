#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Health(pub f32);
#[derive(Debug, Clone, PartialEq, Copy)]

pub struct Food(pub f32);
#[derive(Debug, Clone, PartialEq, Copy)]

pub struct AirLevel(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f32,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
}
