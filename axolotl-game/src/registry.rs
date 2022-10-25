use crate::Error;
use ahash::AHashMap;
use axolotl_api::game::Registry;
use log::warn;
use serde::de::DeserializeOwned;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SimpleRegistry<T> {
    pub key_map: AHashMap<String, usize>,
    pub values: Vec<T>,
    pub next_id: usize,
}
impl<T> SimpleRegistry<T> {
    pub fn new() -> Self {
        Self {
            key_map: Default::default(),
            values: vec![],
            next_id: 0,
        }
    }
}
impl<T: DeserializeOwned> SimpleRegistry<T> {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut registry = SimpleRegistry::new();
        let path = path.as_ref();
        if !path.is_dir() {
            warn!("Path {:?} is not a directory", path);
            return Ok(Self::new());
        }
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
                        Self::load_sub_path(path, v, &mut registry)?;
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
                registry.register(format!("minecraft:{name}"), value);
            } else {
                warn!("Skipping file {:?}", path);
            }
        }
        Ok(registry)
    }
    fn load_sub_path(
        sub_path: impl AsRef<Path>,
        parent: String,
        data: &mut SimpleRegistry<T>,
    ) -> Result<(), Error> {
        for file in std::fs::read_dir(sub_path)? {
            let file = file?;

            let path = file.path();
            if file.file_type()?.is_dir() {
                let name = path.file_name().and_then(|s| s.to_str());
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
                        Some(&s[..s.len() - 5])
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
                    data.register(format!("minecraft:{parent}/{name}"), file);
                } else {
                    warn!("Found a file without a name at {:?}", file.path());
                }
            }
        }
        Ok(())
    }
}

impl<T> Registry<T> for SimpleRegistry<T> {
    fn register(&mut self, namespace: impl Into<String>, item: T) -> usize {
        let namespace = namespace.into();
        let id = self.next_id;
        self.next_id += 1;
        self.key_map.insert(namespace.into(), id);
        self.values.push(item);
        id
    }

    fn register_with_id(&mut self, namespace: impl Into<String>, id: usize, item: T) {
        if id != self.next_id {
            if id < self.next_id {
                warn!("Tried to register an item with an id that is already taken");
            } else {
                todo!("Handle registering an item with an id that is too high");
            }
        }
        self.key_map.insert(namespace.into(), self.next_id);
        self.values.push(item);
        self.next_id += 1;
    }

    fn get_by_id(&self, id: usize) -> Option<&T> {
        self.values.get(id)
    }

    fn get_id(&self, namespace: impl AsRef<str>) -> Option<usize> {
        self.key_map.get(namespace.as_ref()).copied()
    }

    fn get_by_namespace(&self, namespace: impl AsRef<str>) -> Option<&T> {
        self.key_map
            .get(namespace.as_ref())
            .and_then(|id| self.values.get(*id))
    }
}
