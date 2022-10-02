use crate::game::Game;
use crate::NamespacedKey;
use crate::world_gen::noise::density::{BuildDefResult, DensityFunction, DensityState, Function};
use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::Noise;


#[derive(Debug, Clone)]
pub struct AllInCellCache<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> {
    pub function: Function<'function, P>,
}

impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'function,P> for AllInCellCache<'function, P> {
    type FunctionDefinition = Box<FunctionArgument>;

    fn new<G, DS: DensityState>(game: &G, state: &'function DS, def: Self::FunctionDefinition) -> Self where G: Game {
        let function = state.build_from_def(game, *def);
        Self {
            function
        }
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        self.function.compute(state)
    }
    fn build_definition(value: FunctionArgument, _state: &mut impl DensityLoader) -> Result<Self::FunctionDefinition, BuildDefResult> {
        if let FunctionArgument::Function { name, mut arguments } = value {
            if name.get_key().eq("two_d_cache") {
                let argument = arguments.remove("argument").ok_or("argument is required")?;
                Ok(argument)
            } else {
                Err(BuildDefResult::NotFound(FunctionArgument::Function { name, arguments }))
            }
        } else {
            Err(BuildDefResult::NotFound(value))
        }
    }
}
