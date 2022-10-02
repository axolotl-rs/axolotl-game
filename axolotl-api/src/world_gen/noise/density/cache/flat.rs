use crate::game::Game;
use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{
    BuildDefResult, DensityContext, DensityFunction, DensityState, Function,
};
use crate::world_gen::noise::Noise;
use crate::NamespacedKey;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct FlatCache<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    pub function: Function<'function, P>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for FlatCache<'function, P>
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
        FlatCache { function }
    }

    fn compute(&self, state: &impl DensityContext) -> f64 {
        todo!()
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
            if name.get_key().eq("flat_cache") {
                let argument = arguments.remove("argument").ok_or("shift_x is required")?;
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
