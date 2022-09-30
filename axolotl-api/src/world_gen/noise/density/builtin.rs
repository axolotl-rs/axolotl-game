use crate::world_gen::noise::density::{DensityFunction, DensityState};

/// https://minecraft.fandom.com/wiki/Density_function#abs
pub fn abs<DF: DensityFunction, State: DensityState>(state: &State, value: &DF) -> f64 {
    let value = value.compute(state);
    value.abs()
}

///https://minecraft.fandom.com/wiki/Density_function#max
pub fn max<DF: DensityFunction, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    return if one >= two.max() {
        one
    } else {
        let two = two.compute(state);
        one.max(two)
    };
}

/// https://minecraft.fandom.com/wiki/Density_function#min
pub fn min<DF: DensityFunction, Two: DensityFunction, State: DensityState>(
    state: &State,
    one: &DF,
    two: &Two,
) -> f64 {
    let one = one.compute(state);
    return if one <= two.min() {
        one
    } else {
        let two = two.compute(state);
        one.min(two)
    };
}

/// https://minecraft.fandom.com/wiki/Density_function#add
pub fn add<DF: DensityFunction, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    let two = two.compute(state);
    one + two
}

/// https://minecraft.fandom.com/wiki/Density_function#mul
pub fn mul<DF: DensityFunction, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    let two = two.compute(state);
    one * two
}

/// https://minecraft.fandom.com/wiki/Density_function#sub
pub fn cube<DF: DensityFunction, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    one.powi(3)
}

/// https://minecraft.fandom.com/wiki/Density_function#cube
pub fn square<DF: DensityFunction, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    one.powi(2)
}

/// https://minecraft.fandom.com/wiki/Density_function#half_negative
pub fn half_negative<DF: DensityFunction, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    if one < 0.0 {
        one / 2.0
    } else {
        one
    }
}

///https://minecraft.fandom.com/wiki/Density_function#quarter_negative
pub fn quarter_negative<DF: DensityFunction, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    if one < 0.0 {
        one / 4.0
    } else {
        one
    }
}

/// https://minecraft.fandom.com/wiki/Density_function#squeeze
pub fn squeeze<DF: DensityFunction, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    let x = one.clamp(-1.0, 1.0);
    x / 2.0 - x.powi(3) / 24.0
}

pub mod one_param {
    use crate::game::Game;
    use crate::world_gen::noise::density::{DensityFunction, DensityState, Function};
    use crate::world_gen::noise::density::builtin::{
        abs, cube, half_negative, quarter_negative, square, squeeze,
    };

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum OneArgBuiltInFunctionType {
        Abs,
        Cube,
        Square,
        HalfNegative,
        QuarterNegative,
        Squeeze,
    }

    #[derive(Debug, Clone)]
    pub struct OneArgBuiltInFunction<'function> {
        pub fun_type: OneArgBuiltInFunctionType,
        pub param: Function<'function>,
        max: f64,
        min: f64,
    }

    impl<'function> DensityFunction for OneArgBuiltInFunction<'function> {
        type FunctionDefinition = ();

        fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
        where
            G: Game,
        {
            todo!()
        }

        #[inline(always)]
        fn compute<State: DensityState>(&self, state: &State) -> f64 {
            match self.fun_type {
                OneArgBuiltInFunctionType::Abs => abs(state, &self.param),
                OneArgBuiltInFunctionType::Cube => cube(state, &self.param),
                OneArgBuiltInFunctionType::Square => square(state, &self.param),
                OneArgBuiltInFunctionType::HalfNegative => half_negative(state, &self.param),
                OneArgBuiltInFunctionType::QuarterNegative => quarter_negative(state, &self.param),
                OneArgBuiltInFunctionType::Squeeze => squeeze(state, &self.param),
            }
        }
        #[inline(always)]
        fn max(&self) -> f64 {
            self.max
        }
        #[inline(always)]
        fn min(&self) -> f64 {
            self.min
        }
    }
}

pub mod two_param {
    use std::borrow::Cow;

    use serde_json::Value;

    use crate::game::Game;
    use crate::world_gen::noise::density::{
        BuildDefResult, DensityFunction, DensityState, Function,
    };
    use crate::world_gen::noise::density::builtin::{add, max, min, mul};
    use crate::world_gen::noise::density::loading::{DensityLoader, UnloadedFunction};
    use crate::world_gen::noise::density::perlin::Perlin;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum TwoParamBuiltInFunctionType {
        Add,
        Mul,
        Max,
        Min,
    }

    #[derive(Debug, Clone)]
    pub struct TwoParamBuiltInFunction<'function> {
        pub fun_type: TwoParamBuiltInFunctionType,
        pub one: Cow<'function, Function<'function>>,
        pub two: Cow<'function, Function<'function>>,
        max: f64,
        min: f64,
    }

    #[derive(Debug, Clone)]
    pub struct TwoParamDefinition {
        pub fun_type: TwoParamBuiltInFunctionType,
        pub one: UnloadedFunction,
        pub two: UnloadedFunction,
    }

    impl<'function> DensityFunction for TwoParamBuiltInFunction<'function> {
        type FunctionDefinition = TwoParamDefinition;

        fn new<G>(_game: &G, _def: Self::FunctionDefinition) -> Self
        where
            G: Game,
        {
            todo!()
        }
        fn build_definition<State: DensityState>(
            parent: Value,
            state: &mut impl DensityLoader,
        ) -> BuildDefResult<Self::FunctionDefinition> {
            let fun_type = if let Some(value) = parent.as_object() {
                if let Some(fun_type) = value.get("type") {
                    if let Some(fun_type) = fun_type.as_str() {
                        let fun_type = match fun_type {
                            "minecraft:add" => TwoParamBuiltInFunctionType::Add,
                            "minecraft:mul" => TwoParamBuiltInFunctionType::Mul,
                            "minecraft:max" => TwoParamBuiltInFunctionType::Max,
                            "minecraft:min" => TwoParamBuiltInFunctionType::Min,
                            _ => return BuildDefResult::NotFound(parent),
                        };
                        Some(fun_type)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(fun_type) = fun_type {
                let mut value = if let Value::Object(value) = parent {
                    value
                } else {
                    unreachable!()
                };
                let one = state.prep_for_load(value.remove("argument1").unwrap());
                let two = state.prep_for_load(value.remove("argument2").unwrap());
                return BuildDefResult::Ok(TwoParamDefinition { fun_type, one, two });
            }
            BuildDefResult::NotFound(parent)
        }
        #[inline(always)]
        fn compute<State: DensityState>(&self, state: &State) -> f64 {
            match self.fun_type {
                TwoParamBuiltInFunctionType::Add => {
                    add(state, self.one.as_ref(), self.two.as_ref())
                }
                TwoParamBuiltInFunctionType::Mul => {
                    mul(state, self.one.as_ref(), self.two.as_ref())
                }
                TwoParamBuiltInFunctionType::Max => {
                    max(state, self.one.as_ref(), self.two.as_ref())
                }
                TwoParamBuiltInFunctionType::Min => {
                    min(state, self.one.as_ref(), self.two.as_ref())
                }
            }
        }
        #[inline(always)]
        fn max(&self) -> f64 {
            self.max
        }
        #[inline(always)]
        fn min(&self) -> f64 {
            self.min
        }
    }
}
