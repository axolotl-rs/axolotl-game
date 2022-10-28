#![allow(unused)]
extern crate core;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

pub mod chat;
mod color;
pub mod data;
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

pub use axolotl_types::{
    BadNamespacedKeyError, NameSpaceKey, NameSpaceRef, NamespacedKey, OwnedNameSpaceKey,
};

pub trait NumericId {
    fn id(&self) -> usize;
}
impl<NID: NumericId> NumericId for &'_ NID {
    fn id(&self) -> usize {
        (*self).id()
    }
}
impl<NID> NumericId for Arc<NID>
where
    NID: NumericId,
{
    fn id(&self) -> usize {
        (**self).id()
    }
}
impl<NID> NumericId for Box<NID>
where
    NID: NumericId,
{
    fn id(&self) -> usize {
        (**self).id()
    }
}

pub trait NamespacedId {
    fn namespace(&self) -> &str;
    fn key(&self) -> &str;
}
impl<NSI: NamespacedId> NamespacedId for Arc<NSI> {
    fn namespace(&self) -> &str {
        self.as_ref().namespace()
    }

    fn key(&self) -> &str {
        self.as_ref().key()
    }
}
impl<NSI: NamespacedId> NamespacedId for Box<NSI> {
    fn namespace(&self) -> &str {
        self.as_ref().namespace()
    }

    fn key(&self) -> &str {
        self.as_ref().key()
    }
}
impl<NSI> NamespacedId for &'_ NSI
where
    NSI: NamespacedId,
{
    fn namespace(&self) -> &str {
        (*self).namespace()
    }
    fn key(&self) -> &str {
        (*self).key()
    }
}
