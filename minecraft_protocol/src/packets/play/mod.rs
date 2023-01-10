use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use minecraft_protocol_macros::PacketContentType;

use crate::PacketContent;

pub mod client;
pub mod server;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct PlayPing(pub i32);
impl From<i32> for PlayPing {
    fn from(ping: i32) -> Self {
        Self(ping)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PacketContentType)]
pub struct KeepAlive(pub i64);

impl From<i64> for KeepAlive {
    fn from(keep_alive: i64) -> Self {
        Self(keep_alive)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayPluginMessage {
    pub id: Cow<'static, str>,
    pub data: Vec<u8>,
}

impl PacketContent for PlayPluginMessage {}

impl PlayPluginMessage {
    pub fn server_brand(brand: impl Into<String>) -> Self {
        Self {
            id: Cow::Borrowed("minecraft:brand"),
            data: brand.into().into_bytes(),
        }
    }
}
