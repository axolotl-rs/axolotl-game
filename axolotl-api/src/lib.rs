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

pub trait NamespacedKey: Display + Hash {
    fn get_key(&self) -> &str;
    fn get_namespace(&self) -> &str;
}
#[derive(Debug)]
pub struct BadNamespacedKeyError;
impl Display for BadNamespacedKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bad Namespaced Key")
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OwnedNameSpaceKey {
    namespace: String,
    key: String,
}

impl FromStr for OwnedNameSpaceKey {
    type Err = BadNamespacedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let namespace = split.next().ok_or(BadNamespacedKeyError)?;
        let key = split.next().ok_or(BadNamespacedKeyError)?;
        Ok(Self {
            namespace: namespace.to_string(),
            key: key.to_string(),
        })
    }
}

impl TryFrom<String> for OwnedNameSpaceKey {
    type Error = BadNamespacedKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Serialize for OwnedNameSpaceKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct NameSpaceKeyVisitor;

impl<'de> Visitor<'de> for NameSpaceKeyVisitor {
    type Value = OwnedNameSpaceKey;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        OwnedNameSpaceKey::from_str(v).map_err(Error::custom)
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        OwnedNameSpaceKey::from_str(&v).map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for OwnedNameSpaceKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NameSpaceKeyVisitor)
    }
}
impl OwnedNameSpaceKey {
    pub fn new(namespace: String, key: String) -> Self {
        Self { namespace, key }
    }
}

impl Display for OwnedNameSpaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl NamespacedKey for OwnedNameSpaceKey {
    fn get_key(&self) -> &str {
        &self.key
    }
    fn get_namespace(&self) -> &str {
        &self.namespace
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NameSpaceRef<'a> {
    namespace: &'a str,
    key: &'a str,
}

impl<'a> NameSpaceRef<'a> {
    pub fn new(namespace: &'a str, key: &'a str) -> Self {
        Self { namespace, key }
    }
}

impl<'a> Display for NameSpaceRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl<'a> NamespacedKey for NameSpaceRef<'a> {
    fn get_key(&self) -> &str {
        self.key
    }
    fn get_namespace(&self) -> &str {
        self.namespace
    }
}
