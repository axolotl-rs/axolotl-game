use crate::game::Game;
use crate::world_gen::chunk::{into_condensed_location, into_condensed_location_i32};
use crate::world_gen::noise::density::cache::AtomicF64;
use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{
    BuildDefResult, DensityContext, DensityFunction, DensityState, Function,
};
use crate::world_gen::noise::Noise;
use crate::NamespacedKey;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TwoDCache<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    pub function: Function<'function, P>,
    pub cache: Arc<AtomicF64>,
    pub last_value: Arc<AtomicU64>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for TwoDCache<'function, P>
{
    type FunctionDefinition = Box<FunctionArgument>;

    fn new<G, DS: DensityState>(
        game: &G,
        state: &'function DS,
        def: Self::FunctionDefinition,
    ) -> Self
    where
        G: Game,
    {
        let function = state.build_from_def(game, *def);
        Self {
            function,
            cache: Arc::new(AtomicF64::new(0.0)),
            last_value: Arc::new(Default::default()),
        }
    }

    fn compute(&self, state: &impl DensityContext) -> f64 {
        let i = into_condensed_location_i32(state.get_x(), state.get_z());
        if self.last_value.load(Ordering::Relaxed) == i {
            self.cache.load(Ordering::Relaxed)
        } else {
            let value = self.function.compute(state);
            self.cache.store(value, Ordering::Relaxed);
            self.last_value.store(i, Ordering::Relaxed);
            value
        }
    }
    fn build_definition(
        value: FunctionArgument,
        _state: &mut impl DensityLoader,
    ) -> Result<Self::FunctionDefinition, BuildDefResult> {
        if let FunctionArgument::Function {
            name,
            mut arguments,
        } = value
        {
            if name.get_key().eq("two_d_cache") {
                let argument = arguments.remove("argument").ok_or("argument is required")?;
                Ok(argument)
            } else {
                Err(BuildDefResult::NotFound(FunctionArgument::Function {
                    name,
                    arguments,
                }))
            }
        } else {
            Err(BuildDefResult::NotFound(value))
        }
    }
}
