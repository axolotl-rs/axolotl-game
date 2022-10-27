use axolotl_api::chat::{ChatType, ChatTypeDecoration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AxolotlChatType {
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}
impl ChatType for AxolotlChatType {
    fn get_chat(&self) -> &ChatTypeDecoration {
        &self.chat
    }
    fn get_narration(&self) -> &ChatTypeDecoration {
        &self.narration
    }
}
