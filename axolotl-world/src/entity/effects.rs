use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Effect {
    pub ambient: bool,
    pub amplifier: bool,
    pub duration: i32,
    pub id: i32,
    pub show_icon: bool,
    pub show_particles: bool,
}
