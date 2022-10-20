use crate::Error;
use ahash::AHashMap;
use axolotl_api::game::Registry;
use axolotl_api::OwnedNameSpaceKey;
use log::warn;
use serde::de::DeserializeOwned;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SimpleRegistry<T: DeserializeOwned> {
    pub map: AHashMap<String, T>,
}

impl<T: DeserializeOwned> SimpleRegistry<T> {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        if !path.is_dir() {
            warn!("Path {:?} is not a directory", path);
            return Ok(Self::new());
        }
        let mut map = AHashMap::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().and_then(|s| s.to_str()).and_then(|s| {
                if s.ends_with(".json") {
                    Some(s[..s.len() - 5].to_string())
                } else {
                    Some(s.to_string())
                }
            });

            if entry.file_type()?.is_dir() {
                match name {
                    None => {
                        warn!("Found a directory without a name at {:?}", path);
                        continue;
                    }
                    Some(v) => {
                        Self::load_sub_path(path, v, &mut map)?;
                    }
                }
            } else if let Some(name) = name {
                let file = std::fs::File::open(&path)?;
                let value = match serde_json::from_reader(file) {
                    Ok(v) => v,
                    Err(e) => {
                        warn!("Failed to parse file {:?}: {}", path, e);
                        continue;
                    }
                };
                map.insert(format!("minecraft:{name}"), value);
            } else {
                warn!("Skipping file {:?}", path);
            }
        }
        Ok(Self { map })
    }
    fn load_sub_path(
        sub_path: impl AsRef<Path>,
        parent: String,
        data: &mut AHashMap<String, T>,
    ) -> Result<(), Error> {
        for file in std::fs::read_dir(sub_path)? {
            let file = file?;

            let path = file.path();
            if file.file_type()?.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .and_then(|s| Some(s.to_string()));
                if let Some(name) = name {
                    Self::load_sub_path(
                        &path,
                        format!("{parent}/{name}", parent = parent, name = name),
                        data,
                    )?;
                } else {
                    warn!("Found a directory without a name at {:?}", file.path());
                }
            } else {
                let name = path.file_name().and_then(|s| s.to_str()).and_then(|s| {
                    if s.ends_with(".json") {
                        Some(s[..s.len() - 5].to_string())
                    } else {
                        warn!("Non json file {:?}", path);
                        None
                    }
                });
                if let Some(name) = name {
                    let file = std::fs::File::open(&path)?;
                    let file = match serde_json::from_reader(file) {
                        Ok(v) => v,
                        Err(e) => {
                            warn!("Failed to parse file {:?}: {}", path, e);
                            continue;
                        }
                    };
                    data.insert(format!("minecraft:{parent}/{name}"), file);
                } else {
                    warn!("Found a file without a name at {:?}", file.path());
                }
            }
        }
        Ok(())
    }
    pub fn new() -> Self {
        Self {
            map: AHashMap::new(),
        }
    }
}

impl<T: DeserializeOwned> Registry<T> for SimpleRegistry<T> {
    fn register(&mut self, namespace: OwnedNameSpaceKey, item: T) {
        self.map.insert(namespace.to_string(), item);
    }

    fn get(&self, key: &OwnedNameSpaceKey) -> Option<&T> {
        self.map.get(&key.to_string())
    }
}
