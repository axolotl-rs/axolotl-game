use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait ChatType: Debug {
    fn get_chat(&self) -> &ChatTypeDecoration;

    fn get_narration(&self) -> &ChatTypeDecoration;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatTypeDecoration {
    pub parameters: Vec<Parameter>,
    pub translation_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Parameter {
    Sender,
    Target,
    Content,
}
