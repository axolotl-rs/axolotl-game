macro_rules! define_group_def {
    ($name:ident, $($ty_name:ident, $tp:tt),*) => {
        #[derive(Debug, Clone)]
        pub enum $name<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
            $($ty_name(<$tp<'function, P> as DensityFunction<'function, P>>::FunctionDefinition)),*
        }
    };
}
pub(crate) use define_group_def;

macro_rules! define_group {
    ($name:ident,$defs:tt, $($ty_name:ident, $tp:tt, $key:literal),*) => {
        #[derive(Debug, Clone)]
        pub enum $name<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
            $(
              $ty_name($tp<'function, P>)
            ),*
        }

        impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
            for $name<'function, P>
        {
            type FunctionDefinition = $defs<'function, P>;

            fn new<G, DS: DensityState<Perlin = P>>(
                game: &G,
                state: &'function DS,
                def: Self::FunctionDefinition,
            ) -> Self
            where
                G: Game,
            {
                match def {
                    $(
                        $defs::$ty_name(def) => {
                            $name::$ty_name($tp::<P>::new(game, state, def))
                        }
                    ),*
                }
            }
            #[inline(always)]
            fn max(&self) -> f64 {
                match self {
                    $(
                        $name::$ty_name(def) => def.max()
                    ),*
                }
            }
            #[inline(always)]
            fn min(&self) -> f64 {
                match self {
                    $(
                        $name::$ty_name(def) => def.min()
                    ),*
                }
            }
            #[inline(always)]
            fn compute<State: DensityState>(&self, state: &State) -> f64 {
                match self {
                      $(
                            $name::$ty_name(fun) => DensityFunction::compute(fun, state)
                      ),*
                 }
            }
            fn build_definition(
                value: FunctionArgument,
                state: &mut impl DensityLoader,
            ) -> Result<Self::FunctionDefinition, BuildDefResult> {
                if let Some(key) = value.get_function_key() {
                    match key.get_key() {
                        $(
                            $key => {
                                let def = $tp::<P>::build_definition(value, state)?;
                                Ok($defs::$ty_name(def))
                            }
                        ),*
                        _ => Err(BuildDefResult::NotFound(value))
                    }
                } else {
                    Err(BuildDefResult::NotFound(value))
                }
            }
        }
    };
}
pub(crate) use define_group;
