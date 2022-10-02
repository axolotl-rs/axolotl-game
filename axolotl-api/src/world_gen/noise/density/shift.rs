use std::fmt::Debug;
use std::marker::PhantomData;

use crate::game::{DataRegistries, Game, Registry};
use crate::world_gen::dimension::Value;
use crate::world_gen::noise::density::groups::{define_group, define_group_def};
use crate::world_gen::noise::density::loading::{
    get_constant, get_noise, DensityLoader, FunctionArgument,
};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{BuildDefResult, DensityFunction, DensityState, Function};
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};
use crate::{NamespacedKey, OwnedNameSpaceKey};
macro_rules! define_as_noise {
    ($tp:tt,$sel:ident $get_perlin:block, $sel_two:ident $get_noise:block) => {
        impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> NoiseFunction<'function, P>
            for $tp<'function, P>
        {
            fn get_perlin(&$sel) -> &P $get_perlin

            fn get_noise(&$sel_two) -> &Noise $get_noise
        }
    };
}

define_group_def!(
    NoiseFunctionsDef,
    Shift,
    Shift,
    ShiftA,
    ShiftA,
    ShiftB,
    ShiftB,
    ShiftedNoise,
    ShiftedNoise
);
define_group!(
    NoiseFunctions,
    NoiseFunctionsDef,
    Shift,
    Shift,
    "shift",
    ShiftA,
    ShiftA,
    "shift_a",
    ShiftB,
    ShiftB,
    "shift_b",
    ShiftedNoise,
    ShiftedNoise,
    "shifted_noise"
);

define_as_noise!(
    NoiseFunctions,
    self {
        match self {
            NoiseFunctions::Shift(f) => f.get_perlin(),
            NoiseFunctions::ShiftA(f) => f.get_perlin(),
            NoiseFunctions::ShiftB(f) => f.get_perlin(),

            NoiseFunctions::ShiftedNoise(f) => f.get_perlin(),
        }
    },
    self{
        match self {
            NoiseFunctions::Shift(f) => f.get_noise(),
            NoiseFunctions::ShiftA(f) => f.get_noise(),
            NoiseFunctions::ShiftB(f) => f.get_noise(),

            NoiseFunctions::ShiftedNoise(f) => f.get_noise(),
        }
    }
);
macro_rules! generic_new_noise {
    () => {
        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
        where
            G: Game,
        {
            let noise = get_noise!(def, game);

            let value = P::new(state.seed(), noise);
            Self {
                perlin: value,
                phantom: Default::default(),
            }
        }
    };
}
/// A type of density function that works with a perlin noise generator.
pub trait NoiseFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>>:
    Debug + DensityFunction<'function, P>
{
    fn get_perlin(&self) -> &P;

    fn get_noise(&self) -> &Noise;

    #[inline(always)]
    fn compute(&self, x: f64, y: f64, z: f64) -> f64 {
        return self.get_perlin().get(x * 0.25, y * 0.25, z * 0.25) * 4.0;
    }
}

#[derive(Debug, Clone)]
pub struct ShiftB<'function, P: Perlin> {
    perlin: P,
    phantom: PhantomData<&'function ()>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for ShiftB<'function, P>
{
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    generic_new_noise!();
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x() as f64, state.get_y() as f64, 0.0)
    }
}

define_as_noise!(
    ShiftB,
    self {
        &self.perlin
    },
    self{
        self.perlin.get_setting()
    }
);
#[derive(Debug, Clone)]
pub struct ShiftA<'function, P: Perlin> {
    perlin: P,
    phantom: PhantomData<&'function ()>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for ShiftA<'function, P>
{
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    generic_new_noise!();

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(self, state.get_x() as f64, 0.0, state.get_z() as f64)
    }
}
define_as_noise!(
    ShiftA,
    self {
        &self.perlin
    },
    self{
        self.perlin.get_setting()
    }
);
#[derive(Debug, Clone)]
pub struct Shift<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    perlin: P,
    phantom: PhantomData<&'function ()>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for Shift<'function, P>
{
    type FunctionDefinition = NameSpaceKeyOrType<Noise>;

    generic_new_noise!();

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        <Self as NoiseFunction<P>>::compute(
            self,
            state.get_x() as f64,
            state.get_y() as f64,
            state.get_z() as f64,
        )
    }
}
define_as_noise!(
    Shift,
    self {
        &self.perlin
    },
    self{
        self.perlin.get_setting()
    }
);
#[derive(Debug, Clone)]
pub struct ShiftedNoiseLayout {
    pub noise: NameSpaceKeyOrType<Noise>,
    xz_scale: f64,
    y_scale: f64,
    shift_x: Box<FunctionArgument>,
    shift_y: Box<FunctionArgument>,
    shift_z: Box<FunctionArgument>,
}

#[derive(Debug, Clone)]
pub struct ShiftedNoise<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    perlin: P,
    xz_scale: f64,
    y_scale: f64,
    shift_x: Function<'function, P>,
    shift_y: Function<'function, P>,
    shift_z: Function<'function, P>,
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for ShiftedNoise<'function, P>
{
    type FunctionDefinition = ShiftedNoiseLayout;

    fn new<G, DS: DensityState<Perlin = P>>(
        game: &G,
        state: &'function DS,
        def: ShiftedNoiseLayout,
    ) -> Self
    where
        G: Game,
    {
        let noise = get_noise!(def.noise, game);
        let value = P::new(state.seed(), noise);
        Self {
            perlin: value,
            xz_scale: def.xz_scale,
            y_scale: def.y_scale,
            shift_x: state.build_from_def(game, *def.shift_x),
            shift_y: state.build_from_def(game, *def.shift_y),
            shift_z: state.build_from_def(game, *def.shift_z),
        }
    }

    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        let x = state.get_x() as f64 * self.xz_scale + self.shift_x.compute(state);
        let y = state.get_y() as f64 * self.y_scale + self.shift_y.compute(state);
        let z = state.get_z() as f64 * self.xz_scale + self.shift_z.compute(state);
        <Self as NoiseFunction<P>>::compute(self, x, y, z)
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
            if name.get_key().eq("shifted_noise") {
                let xz_scale = get_constant!(arguments, "xz_scale");
                let y_scale = get_constant!(arguments, "y_scale");
                let shift_x = arguments.remove("shift_x").ok_or("shift_x is required")?;
                let shift_y = arguments.remove("shift_y").ok_or("shift_y is required")?;
                let shift_z = arguments.remove("shift_z").ok_or("shift_z is required")?;
                let noise = match *arguments.remove("noise").ok_or("noise is required")? {
                    FunctionArgument::Noise(noise) => noise,
                    _ => {
                        return Err("noise must be a noise".into());
                    }
                };
                Ok(ShiftedNoiseLayout {
                    noise,
                    xz_scale,
                    y_scale,
                    shift_x,
                    shift_y,
                    shift_z,
                })
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
define_as_noise!(
    ShiftedNoise,
    self {
        &self.perlin
    },
    self{
        self.perlin.get_setting()
    }
);
