#![allow(unused)]
extern crate core;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

mod color;
pub mod effects;
pub mod enchantments;
pub mod events;
pub mod game;
pub mod item;
pub mod math;
pub mod name;
pub mod player;
pub mod world;
pub mod world_gen;

pub use axolotl_types::{BadNamespacedKeyError, NameSpaceRef, NamespacedKey, OwnedNameSpaceKey};
