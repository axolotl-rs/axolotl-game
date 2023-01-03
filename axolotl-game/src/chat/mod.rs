use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use axolotl_api::chat::{ChatType, ChatTypeDecoration};
use axolotl_api::data::{ForPacket, GenericPacketVersion};
use axolotl_api::NameSpaceKey;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AxolotlChatType {
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}

impl ForPacket for AxolotlChatType {
    type PacketVersion<'p>
    = GenericPacketVersion<'p, Self>    where
    Self: 'p;

    fn as_packet_version<'p>(
        &'p self,
        id: usize,
        namespace: impl Into<NameSpaceKey<'p>>,
    ) -> Self::PacketVersion<'p> {
        GenericPacketVersion {
            id,
            name: namespace.into(),
            element: Cow::Borrowed(self),
        }
    }
}

impl ChatType for AxolotlChatType {
    fn get_chat(&self) -> &ChatTypeDecoration {
        &self.chat
    }
    fn get_narration(&self) -> &ChatTypeDecoration {
        &self.narration
    }
}
