use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::{DensityFunction, DensityState};
use crate::world_gen::noise::Noise;
macro_rules! define_simple_function {
    ($name:ident,$value:ident $calculate:block) => {
        pub fn $name<
            'function,
            P: Perlin<Noise = Noise, Seed = [u8; 16]>,
            DF: DensityFunction<'function, P>,
            State: DensityState,
        >(
            state: &State,
            value: &DF,
        ) -> f64 {
            let $value = value.compute(state);
            $calculate
        }
    };
        ($name:ident,$one:ident,$two:ident, $state:ident, $calculate:block) => {
        pub fn $name<'function,
            P: Perlin<Noise = Noise, Seed = [u8; 16]>,
            DF: DensityFunction<'function, P>,
            State: DensityState,
        >(
            $state: &State,
            $one: &DF,
            $two: &DF,

        ) -> f64 $calculate
    };
}

/// https://minecraft.fandom.com/wiki/Density_function#abs
define_simple_function!(abs, value { value.abs() });

///https://minecraft.fandom.com/wiki/Density_function#max
define_simple_function!(max, one, two, state, {
    let one = one.compute(state);
    return if one >= two.max() {
        one
    } else {
        let two = two.compute(state);
        one.max(two)
    };
});

/// https://minecraft.fandom.com/wiki/Density_function#min
define_simple_function!(min, one, two, state, {
    let one = one.compute(state);
    return if one <= two.min() {
        one
    } else {
        let two = two.compute(state);
        one.min(two)
    };
});
/// https://minecraft.fandom.com/wiki/Density_function#add
define_simple_function!(add, one, two, state, {
    let one = one.compute(state);
    let two = two.compute(state);
    one + two
});

/// https://minecraft.fandom.com/wiki/Density_function#mul
define_simple_function!(mul, one, two, state, {
    let one = one.compute(state);
    let two = two.compute(state);
    one * two
});

/// https://minecraft.fandom.com/wiki/Density_function#cube
///
define_simple_function!(cube, value {
    value * value * value
});

/// https://minecraft.fandom.com/wiki/Density_function#square
define_simple_function!(square, value {
    value * value
});

/// https://minecraft.fandom.com/wiki/Density_function#half_negative
define_simple_function!(half_negative, value {
    if value < 0.0 {
        value / 2.0
    } else {
        value
    }
});

///https://minecraft.fandom.com/wiki/Density_function#quarter_negative
define_simple_function!(quarter_negative, value {
    if value < 0.0 {
        value / 4.0
    } else {
        value
    }
});
/// https://minecraft.fandom.com/wiki/Density_function#squeeze
define_simple_function!(squeeze, value {
    let x = value.clamp(-1.0, 1.0);
    x / 2.0 - x.powi(3) / 24.0
});

pub mod one_param {
    use crate::game::Game;
    use crate::world_gen::noise::density::builtin::{
        abs, cube, half_negative, quarter_negative, square, squeeze,
    };
    use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
    use crate::world_gen::noise::density::perlin::Perlin;
    use crate::world_gen::noise::density::{
        BuildDefResult, DensityFunction, DensityState, Function,
    };
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
    pub struct OneArgBuiltInFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
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

    impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
        for OneArgBuiltInFunction<'function, P>
    {
        type FunctionDefinition = OneParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
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
        fn build_definition(
            value: FunctionArgument,
            _state: &mut impl DensityLoader,
        ) -> Result<Self::FunctionDefinition, BuildDefResult> {
            if let FunctionArgument::Function {
                name,
                mut arguments,
            } = value
            {
                match name.key.as_str() {
                    "abs" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::Abs,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    "cube" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::Cube,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    "square" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::Square,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    "half_negative" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::HalfNegative,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    "quarter_negative" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::QuarterNegative,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    "squeeze" => Ok(OneParamDefinition {
                        fun_type: OneArgBuiltInFunctionType::Squeeze,
                        one: arguments.remove("argument").unwrap(),
                    }),
                    _ => Err(BuildDefResult::NotFound(FunctionArgument::Function {
                        name,
                        arguments,
                    })),
                }
            } else {
                return Err(BuildDefResult::NotFound(value));
            }
        }
    }
}

pub mod two_param {
    use std::borrow::Cow;

    use serde_json::Value;

    use crate::game::Game;
    use crate::world_gen::noise::density::builtin::{add, max, min, mul};
    use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
    use crate::world_gen::noise::density::perlin::Perlin;
    use crate::world_gen::noise::density::{
        BuildDefResult, DensityFunction, DensityState, Function,
    };
    use crate::world_gen::noise::Noise;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum TwoParamBuiltInFunctionType {
        Add,
        Mul,
        Max,
        Min,
    }

    #[derive(Debug, Clone)]
    pub struct TwoParamBuiltInFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
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

    impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
        for TwoParamBuiltInFunction<'function, P>
    {
        type FunctionDefinition = TwoParamDefinition;

        fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self
        where
            G: Game,
        {
            todo!()
        }

        fn build_definition(
            parent: FunctionArgument,
            state: &mut impl DensityLoader,
        ) -> Result<Self::FunctionDefinition, BuildDefResult> {
            if let FunctionArgument::Function {
                name,
                mut arguments,
            } = parent
            {
                match name.key.as_str() {
                    "add" => Ok(TwoParamDefinition {
                        fun_type: TwoParamBuiltInFunctionType::Add,
                        one: arguments.remove("argument1").unwrap(),
                        two: arguments.remove("argument2").unwrap(),
                    }),
                    "mul" => Ok(TwoParamDefinition {
                        fun_type: TwoParamBuiltInFunctionType::Add,
                        one: arguments.remove("argument1").unwrap(),
                        two: arguments.remove("argument2").unwrap(),
                    }),
                    "max" => Ok(TwoParamDefinition {
                        fun_type: TwoParamBuiltInFunctionType::Add,
                        one: arguments.remove("argument1").unwrap(),
                        two: arguments.remove("argument2").unwrap(),
                    }),
                    "min" => Ok(TwoParamDefinition {
                        fun_type: TwoParamBuiltInFunctionType::Add,
                        one: arguments.remove("argument1").unwrap(),
                        two: arguments.remove("argument2").unwrap(),
                    }),
                    _ => Err(BuildDefResult::NotFound(FunctionArgument::Function {
                        name,
                        arguments,
                    })),
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
