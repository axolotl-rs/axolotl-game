use serde::{Deserialize, Serialize};

use crate::world_gen::biome::{Biome, Carvers, Effects, Features, Spawners};
use crate::world_gen::Precipitation;
use crate::OwnedNameSpaceKey;

#[derive(Debug, Serialize, Deserialize)]
pub enum VanillaPrecipitation {
    Rain,
    Snow,
    None,
}

impl Precipitation for VanillaPrecipitation {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPackBiome {
    #[serde(skip)]
    pub namespace: Option<OwnedNameSpaceKey>,
    pub carvers: Carvers,
    pub downfall: f32,
    pub effects: Effects,
    //pub features: Features,
    pub precipitation: VanillaPrecipitation,
    //pub spawn_costs: ,
    pub spawners: Spawners,
    pub temperature: f32,
}

impl Biome for DataPackBiome {
    type Precipitation = VanillaPrecipitation;

    fn get_namespace(&self) -> &OwnedNameSpaceKey {
        self.namespace.as_ref().unwrap()
    }

    fn carvers(&self) -> &Carvers {
        todo!()
    }

    fn get_downfall(&self) -> f32 {
        self.downfall
    }

    fn get_effects(&self) -> &Effects {
        &self.effects
    }

    fn get_precipitation(&self) -> &Self::Precipitation {
        &self.precipitation
    }

    fn features(&self) -> &Features {
        todo!()
    }

    fn creature_spawn_probabilities(&self) -> f32 {
        todo!()
    }

    fn spawners(&self) -> &Spawners {
        todo!()
    }

    fn temperature(&self) -> f32 {
        todo!()
    }
}

#[derive(Debug)]
pub enum DynamicBiome {
    Compiled(Box<dyn Biome<Precipitation = VanillaPrecipitation>>),
    DataPack(DataPackBiome),
}
