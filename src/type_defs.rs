//use fixed::prelude::*;
use fixed::types::I48F16;
//use fixed_macro::fixed;

// define fixed point used
pub type FP = I48F16;

// EPSILON is equal to the "error" in precision, or the "step" between numbers in FP
pub fn EPSILON() -> FP {
    FP::from_num(1 / (2 ^ 16))
}

pub fn to_fp(num: impl fixed::traits::ToFixed) -> FP {
    FP::from_num(num)
}
