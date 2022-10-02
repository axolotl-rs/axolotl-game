use crate::game::Game;
use crate::math;
use crate::math::{lerp, linear_extend};
use crate::world_gen::noise::density::{DensityContext, DensityFunction, DensityState, Function};
use serde::de::{DeserializeOwned, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fmt::Formatter;

use crate::world_gen::noise::density::loading::FunctionArgument;
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::{NameSpaceKeyOrType, Noise};

#[derive(Debug, Clone)]
pub enum SplineFunction<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> {
    Spline {
        function: Function<'function, P>,
        derivatives: Vec<f64>,
        locations: Vec<f64>,
        values: Vec<SplineOrConstant<SplineFunction<'function, P>>>,
        min: f64,
        max: f64,
    },
    Constant(f64),
}

impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> SplineFunction<'function, P> {
    /// TODO Needs to be rewritten to be less ugly
    /// TODO add documentation
    fn calculate_min_max(
        function: &Function<'function, P>,
        locations: &Vec<f64>,
        values: &Vec<SplineOrConstant<SplineFunction<'function, P>>>,
        derivatives: &Vec<f64>,
    ) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let function_max = function.max();
        let function_min = function.min();
        if function_min < locations[0] {
            let val_one = linear_extend(
                function_min,
                &locations,
                values.first().unwrap().min(),
                &derivatives,
                0,
            );
            let value_two = linear_extend(
                function_min,
                &locations,
                values.first().unwrap().max(),
                &derivatives,
                0,
            );
            min = min.min(val_one).min(value_two);
            max = max.max(val_one).max(value_two);
        }
        let i = locations.len() - 1;
        if function_max > locations[i] {
            let val_one = linear_extend(function_max, &locations, values[i].min(), &derivatives, i);
            let value_two =
                linear_extend(function_max, &locations, values[i].max(), &derivatives, i);
            min = min.min(val_one).min(value_two);
            max = max.max(val_one).max(value_two);
        }

        for value in values.iter() {
            min = min.min(value.min());
            max = max.max(value.max());
        }
        for m in 0..i {
            let next_m = m + 1;
            let this_derivative = derivatives[m];
            let next_derivative = derivatives[next_m];
            if this_derivative == 0f64 && next_derivative == 0f64 {
                continue;
            }

            let this_location = locations[m];
            let next_location = locations[next_m];
            let location_difference = next_location - this_location;

            let this_function = &values[m];
            let next_function = &values[next_m];
            let this_function_min = this_function.min();
            let this_function_max = this_function.max();
            let next_function_min = next_function.min();
            let next_function_max = next_function.max();

            let v = this_derivative * location_difference;
            let w = next_derivative * location_difference;
            let x = this_function_min.min(next_function_min);
            let y = this_function_max.max(next_function_max);
            let z = v - next_function_max + this_function_min;
            let aa = v - next_function_min + this_function_max;
            let ab = -w - next_function_max + this_function_min;
            let ac = -w - next_function_min + this_function_max;
            let ad = z.min(ab);
            let ae = aa.max(ac);
            min = min.min(x + 0.25f64 * ad);
            max = max.max(y + 0.25f64 * ae);
        }
        (min, max)
    }
}
impl<'function, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for SplineFunction<'function, P>
{
    type FunctionDefinition = Spline;

    fn new<G, DS: DensityState<Perlin = P>>(
        game: &G,
        state: &'function DS,
        mut def: Self::FunctionDefinition,
    ) -> Self
    where
        G: Game,
    {
        let function = state.build_from_def_with_cache::<G, P>(game, def.coordinate);
        let mut points: Vec<Point<Spline>> = def.points;

        let mut locations = Vec::with_capacity(points.len());
        let mut values: Vec<SplineOrConstant<SplineFunction<'function, P>>> =
            Vec::with_capacity(points.len());
        let mut derivatives = Vec::with_capacity(points.len());

        for point in points.into_iter() {
            let value: SplineFunction<'function, P> = match point.value {
                SplineOrConstant::Spline(spline) => {
                    SplineFunction::<'function, P>::new(game, state, *spline)
                }
                SplineOrConstant::Constant(constant) => SplineFunction::Constant(constant),
            };
            derivatives.push(point.derivative);
            locations.push(point.location);
        }

        if derivatives.len() != locations.len() && derivatives.len() != values.len() {
            panic!("Derivatives and locations must be the same length");
        }
        let (min, max) =
            SplineFunction::calculate_min_max(&function, &locations, &values, &derivatives);
        SplineFunction::Spline {
            function,
            derivatives,
            locations,
            values,
            min,
            max,
        }
    }
    #[inline]
    fn compute(&self, state: &impl DensityContext) -> f64 {
        match self {
            SplineFunction::Spline {
                function,
                derivatives,
                locations,
                values,
                ..
            } => {
                let input = function.compute(state);
                let i = math::binary_search(&locations, |v| input < *v);
                if i == 0 {
                    return linear_extend(
                        input,
                        locations,
                        values.first().unwrap().compute(state),
                        derivatives,
                        0,
                    );
                }

                if locations.len() == i {
                    let index = locations.len() - 1;
                    return linear_extend(
                        input,
                        locations,
                        values.get(index).unwrap().compute(state),
                        derivatives,
                        index,
                    );
                } else {
                    let subtracted_i = i - 1;
                    let location_one = *locations.get(subtracted_i).unwrap();
                    let last_location = *locations.last().unwrap();
                    let k = (input - location_one) / (last_location - location_one);
                    let function_one = values.get(subtracted_i).unwrap().compute(state);
                    let last_function = values.last().unwrap().compute(state);
                    let derivative_one = *derivatives.get(subtracted_i).unwrap();
                    let last_derivative = *derivatives.last().unwrap();

                    let p = derivative_one * (last_location - location_one)
                        - (last_function - function_one);
                    let q = -last_derivative * (last_location - location_one)
                        + (last_function - function_one);
                    lerp(k, function_one, last_function) * (1f64 - k) * lerp(k, p, q)
                }
            }
            SplineFunction::Constant(value) => *value,
        }
    }

    #[inline(always)]
    fn max(&self) -> f64 {
        match self {
            SplineFunction::Spline { max, .. } => *max,
            SplineFunction::Constant(value) => *value,
        }
    }
    #[inline(always)]
    fn min(&self) -> f64 {
        match self {
            SplineFunction::Spline { min, .. } => *min,
            SplineFunction::Constant(value) => *value,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spline {
    pub coordinate: NameSpaceKeyOrType<FunctionArgument>,
    pub points: Vec<Point<Spline>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Point<T> {
    pub derivative: f64,
    pub location: f64,
    pub value: SplineOrConstant<T>,
}

#[derive(Debug, Clone)]
pub enum SplineOrConstant<T> {
    Spline(Box<T>),
    Constant(f64),
}
impl<'function, T, P: Perlin<Noise = Noise, Seed = [u8; 16]>> DensityFunction<'function, P>
    for SplineOrConstant<T>
where
    T: DensityFunction<'function, P>,
{
    type FunctionDefinition = ();

    fn new<G, DS: DensityState<Perlin = P>>(
        game: &G,
        state: &'function DS,
        def: Self::FunctionDefinition,
    ) -> Self
    where
        G: Game,
    {
        panic!("Cannot create a SplineOrConstant from a definition")
    }
    #[inline]
    fn compute(&self, state: &impl DensityContext) -> f64 {
        match self {
            SplineOrConstant::Spline(spline) => spline.compute(state),
            SplineOrConstant::Constant(value) => *value,
        }
    }

    #[inline]
    fn max(&self) -> f64 {
        match self {
            SplineOrConstant::Spline(spline) => spline.max(),
            SplineOrConstant::Constant(value) => *value,
        }
    }
    #[inline]
    fn min(&self) -> f64 {
        match self {
            SplineOrConstant::Spline(spline) => spline.min(),
            SplineOrConstant::Constant(value) => *value,
        }
    }
}
struct SplineOrConstantVisitor<T>(std::marker::PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for SplineOrConstantVisitor<T> {
    type Value = SplineOrConstant<T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SplineOrConstant::Constant(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SplineOrConstant::Constant(v))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(SplineOrConstant::Spline(Box::new(
            Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(&mut map))?,
        )))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SplineOrConstant<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SplineOrConstantVisitor(std::marker::PhantomData))
    }
}
