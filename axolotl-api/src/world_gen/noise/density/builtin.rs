use crate::world_gen::noise::density::{DensityFunction, DensityState};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::Noise;

/// https://minecraft.fandom.com/wiki/Density_function#abs
pub fn abs< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF: for<'a> DensityFunction<'a,P>, State: DensityState>(state: &State, value: &DF) -> f64 {
    let value = value.compute(state);
    value.abs()
}

///https://minecraft.fandom.com/wiki/Density_function#max
pub fn max< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF: for<'a> DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    return if one >= two.max() {
        one
    } else {
        let two = two.compute(state);
        one.max(two)
    };
}

/// https://minecraft.fandom.com/wiki/Density_function#min
pub fn min< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF: for<'a>DensityFunction<'a,P>,  Two:for<'a>  DensityFunction<'a,P>, State: DensityState>(
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
pub fn add< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF: for<'a> DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    let two = two.compute(state);
    one + two
}

/// https://minecraft.fandom.com/wiki/Density_function#mul
pub fn mul< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF:for<'a>   DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF, two: &DF) -> f64 {
    let one = one.compute(state);
    let two = two.compute(state);
    one * two
}

/// https://minecraft.fandom.com/wiki/Density_function#sub
pub fn cube< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF:for<'a>  DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    one.powi(3)
}

/// https://minecraft.fandom.com/wiki/Density_function#cube
pub fn square< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF:for<'a>  DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    one.powi(2)
}

/// https://minecraft.fandom.com/wiki/Density_function#half_negative
pub fn half_negative< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF: for<'a>  DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    if one < 0.0 {
        one / 2.0
    } else {
        one
    }
}

///https://minecraft.fandom.com/wiki/Density_function#quarter_negative
pub fn quarter_negative< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF:  for<'a> DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF) -> f64 {
    let one = one.compute(state);
    if one < 0.0 {
        one / 4.0
    } else {
        one
    }
}

/// https://minecraft.fandom.com/wiki/Density_function#squeeze
pub fn squeeze< P: Perlin<Noise=Noise, Seed=[u8; 16]>,DF:for<'a>  DensityFunction<'a,P>, State: DensityState>(state: &State, one: &DF) -> f64 {
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

    impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'function,P> for OneArgBuiltInFunction<'function, P> {
        type FunctionDefinition = OneParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
            todo!()
        }
        fn build_definition(value: FunctionArgument, _state: &mut impl DensityLoader) -> Result<Self::FunctionDefinition, BuildDefResult> {
            if let FunctionArgument::Function { name, mut arguments } = value {
                match name.key.as_str() {
                    "abs" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Abs,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    "cube" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Cube,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    "square" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Square,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    "half_negative" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::HalfNegative,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    "quarter_negative" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::QuarterNegative,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    "squeeze" => {
                        Ok(OneParamDefinition {
                            fun_type: OneArgBuiltInFunctionType::Squeeze,
                            one: arguments.remove("argument").unwrap(),
                        })
                    }
                    _ => {
                        Err(BuildDefResult::NotFound(FunctionArgument::Function {
                            name,
                            arguments,
                        }))
                    }
                }
            } else {
                return Err(BuildDefResult::NotFound(value));
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

    impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'function,P> for TwoParamBuiltInFunction<'function, P> {
        type FunctionDefinition = TwoParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
            todo!()
        }

        fn build_definition(
            parent: FunctionArgument,
            state: &mut impl DensityLoader,
        ) -> Result<Self::FunctionDefinition, BuildDefResult> {
            if let FunctionArgument::Function { name, mut arguments } = parent {
                match name.key.as_str() {
                    "add" => {
                        Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one: arguments.remove("argument1").unwrap(),
                            two: arguments.remove("argument2").unwrap(),
                        })
                    }
                    "mul" => {
                        Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one: arguments.remove("argument1").unwrap(),
                            two: arguments.remove("argument2").unwrap(),
                        })
                    }
                    "max" => {
                        Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one: arguments.remove("argument1").unwrap(),
                            two: arguments.remove("argument2").unwrap(),
                        })
                    }
                    "min" => {
                       Ok(TwoParamDefinition {
                            fun_type: TwoParamBuiltInFunctionType::Add,
                            one: arguments.remove("argument1").unwrap(),
                            two: arguments.remove("argument2").unwrap(),
                        })
                    }
                    _ => {
                        Err(BuildDefResult::NotFound(FunctionArgument::Function {
                            name,
                            arguments,
                        }))
                    }
                }
            } else {
                return Err(BuildDefResult::NotFound(parent));
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
