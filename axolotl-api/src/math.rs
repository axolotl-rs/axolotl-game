use num_traits::{Num, One, Zero};
use std::ops::{Add, Div, Mul, Sub, SubAssign};
pub fn binary_search<N, Test>(array: &Vec<N>, test: Test) -> usize
where
    Test: Fn(&N) -> bool,
{
    let mut min = 0_usize;
    let mut i = array.len();
    while i > 0 {
        let j = i / 2;
        let k = min + j;
        if test(&array[k]) {
            i = j;
            continue;
        }
        min = k + 1_usize;
        i -= j + 1_usize;
    }
    min
}
#[inline(always)]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
#[inline]
pub fn linear_extend(f: f64, fs: &[f64], g: f64, gs: &[f64], i: usize) -> f64 {
    let h = gs[i];
    if h == 0.0 {
        return g;
    }
    g + h * (f - fs[i])
}
