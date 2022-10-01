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
    use crate::world_gen::noise::density::{BuildDefResult, DensityFunction, DensityState, Function};
    use crate::world_gen::noise::density::builtin::{
        abs, cube, half_negative, quarter_negative, square, squeeze,
    };
    use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
    use crate::world_gen::noise::density::perlin::Perlin;
    use crate::world_gen::noise::Noise;

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
    pub struct OneArgBuiltInFunction<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> {
        pub fun_type: OneArgBuiltInFunctionType,
        pub param: Function<'function, P>,
        max: f64,
        min: f64,
    }

    #[derive(Debug, Clone)]
    pub struct OneParamDefinition {
        pub fun_type: OneArgBuiltInFunctionType,
        pub one: Box<FunctionArgument>,
    }

    impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction for OneArgBuiltInFunction<'function, P> {
        type FunctionDefinition = OneParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
            todo!()
        }
        fn build_definition<'function, State: DensityState>(value: FunctionArgument, _state: &mut impl DensityLoader) -> BuildDefResult<Self::FunctionDefinition> {
            if let FunctionArgument::Function { name, arguments } = parent {
                match name.key.as_str() {
                    "abs" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Abs,
                            one: arguments,
                        })
                    }
                    "cube" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Cube,
                            one: arguments,
                        })
                    }
                    "square" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Square,
                            one: arguments,
                        })
                    }
                    "half_negative" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::HalfNegative,
                            one: arguments,
                        })
                    }
                    "quarter_negative" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::QuarterNegative,
                            one: arguments,
                        })
                    }
                    "squeeze" => {
                        BuildDefResult::Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Squeeze,
                            one: arguments,
                        })
                    }
                    _ => {
                        BuildDefResult::NotFound(FunctionArgument::Function {
                            name,
                            arguments,
                        })
                    }
                }
            } else {
                return BuildDefResult::NotFound(parent);
            }
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
    use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
    use crate::world_gen::noise::density::perlin::Perlin;
    use crate::world_gen::noise::Noise;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum TwoParamBuiltInFunctionType {
        Add,
        Mul,
        Max,
        Min,
    }

    #[derive(Debug, Clone)]
    pub struct TwoParamBuiltInFunction<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> {
        pub fun_type: TwoParamBuiltInFunctionType,
        pub one: Cow<'function, Function<'function, P>>,
        pub two: Cow<'function, Function<'function, P>>,
        max: f64,
        min: f64,
    }

    #[derive(Debug, Clone)]
    pub struct TwoParamDefinition {
        pub fun_type: TwoParamBuiltInFunctionType,
        pub one: Box<FunctionArgument>,
        pub two: Box<FunctionArgument>,
    }

    impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction for TwoParamBuiltInFunction<'function, P> {
        type FunctionDefinition = TwoParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
            todo!()
        }

        fn build_definition<State: DensityState>(
            parent: FunctionArgument,
            state: &mut impl DensityLoader,
        ) -> BuildDefResult<Self::FunctionDefinition> {
            if let FunctionArgument::TwoArgumentFunction { name, arguments } = parent {
                match name.key.as_str() {
                    "add" => {
                        let (one, two) = arguments;
                        BuildDefResult::Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one,
                            two,
                        })
                    }
                    "mul" => {
                        let (one, two) = arguments;
                        BuildDefResult::Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one,
                            two,
                        })
                    }
                    "max" => {
                        let (one, two) = arguments;
                        BuildDefResult::Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one,
                            two,
                        })
                    }
                    "min" => {
                        let (one, two) = arguments;
                        BuildDefResult::Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one,
                            two,
                        })
                    }
                    _ => {
                        BuildDefResult::NotFound(FunctionArgument::TwoArgumentFunction {
                            name,
                            arguments,
                        })
                    }
                }
            } else {
                return BuildDefResult::NotFound(parent);
            }
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
