#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Health(pub f32);

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Food(pub f32);

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct AirLevel(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }
    pub fn update_from_ref(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
        self.yaw = other.yaw;
        self.pitch = other.pitch;
    }
    pub fn update(&mut self, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.yaw = yaw;
        self.pitch = pitch;
    }
    pub fn update_location(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn update_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
    }
}
