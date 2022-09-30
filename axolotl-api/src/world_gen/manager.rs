use std::error::Error;
use std::fmt::Debug;

use uuid::Uuid;

use crate::world::World;

pub trait WorldGenerator: Debug {}

pub trait WorldCreator: Default {
    fn new() -> Self
    where
        Self: Sized;
    fn set_name(self, name: impl Into<String>) -> Self;
    fn set_seed(self, seed: i64) -> Self;
    fn generator(self, generator: impl WorldGenerator + 'static) -> Self;
}

pub trait WorldManager {
    type WorldType: World;
    type WorldCreator: WorldCreator;
    type Error: Error;

    fn get_world_by_name(&self, name: &str) -> Option<&Self::WorldType>;

    fn get_world_uuid(&self, uuid: &Uuid) -> Option<&Self::WorldType>;

    fn create_world(
        &mut self,
        creator: Self::WorldCreator,
    ) -> Result<&Self::WorldType, Self::Error>;
}

#[cfg(test)]
pub mod test {
    use crate::world_gen::manager::WorldGenerator;

    #[derive(Debug, Default)]
    pub struct TestWorldCreator {
        name: Option<String>,
        seed: Option<i64>,
        generator: Option<Box<dyn WorldGenerator>>,
    }

    impl super::WorldCreator for TestWorldCreator {
        fn new() -> Self
        where
            Self: Sized,
        {
            Self {
                name: None,
                seed: None,
                generator: None,
            }
        }
        fn set_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }
        fn set_seed(mut self, seed: i64) -> Self {
            self.seed = Some(seed);
            self
        }
        fn generator(mut self, generator: impl WorldGenerator + 'static) -> Self {
            self.generator = Some(Box::new(generator));
            self
        }
    }
}
