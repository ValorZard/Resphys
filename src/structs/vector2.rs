use fixed::traits::ToFixed;
use fixed_sqrt::FixedSqrt;
use num_traits::{abs, clamp_max, clamp_min, Float, PrimInt};
use std::fmt;

use std::ops::{Add, Div, Mul, Sub};

use crate::{to_fp, FP};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: FP,
    pub y: FP,
}

impl Vec2 {
    pub fn new(x: FP, y: FP) -> Self {
        Self { x: x, y: y }
    }

    pub fn x(&self) -> FP {
        self.x
    }

    pub fn y(&self) -> FP {
        self.y
    }

    pub fn x_mut(&mut self) -> &mut FP {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut FP {
        &mut self.y
    }

    pub fn set_x(&mut self, value: impl fixed::traits::ToFixed) {
        self.x = to_fp(value);
    }

    pub fn set_y(&mut self, value: impl fixed::traits::ToFixed) {
        self.y = to_fp(value);
    }

    pub fn zero() -> Self {
        Self::new(FP::from_num(0), FP::from_num(0))
    }

    pub fn one() -> Self {
        Self::new(FP::from_num(1), FP::from_num(1))
    }

    pub fn unit_x() -> Self {
        Self::new(FP::from_num(1.0), FP::from_num(0))
    }

    pub fn unit_y() -> Self {
        Self::new(FP::from_num(0), FP::from_num(1.0))
    }

    pub fn from<T: ToFixed>(x: T, y: T) -> Self {
        Self::new(FP::from_num(x), FP::from_num(y))
    }

    pub fn from_float<T: Float + ToFixed>(x: T, y: T) -> Self {
        Self::new(FP::from_num(x), FP::from_num(y))
    }

    pub fn from_int<T: PrimInt + ToFixed>(x: T, y: T) -> Self {
        Self::new(FP::from_num(x), FP::from_num(y))
    }

    // Creates a vector with all elements set to value
    pub fn splat(value: FP) -> Self {
        Self::new(value, value)
    }

    pub fn min(self, other: Self) -> Self {
        Self::new(clamp_min(self.x, other.x), clamp_min(self.y, other.y))
    }

    pub fn max(self, other: Self) -> Self {
        Self::new(clamp_max(self.x, other.x), clamp_max(self.y, other.y))
    }

    pub fn add_scalar(self, value: impl fixed::traits::ToFixed) -> Self {
        let fixed_value = to_fp(value);
        Self::new(self.x + fixed_value, self.y + fixed_value)
    }

    pub fn sub_scalar(self, value: impl fixed::traits::ToFixed) -> Self {
        let fixed_value = to_fp(value);
        Self::new(self.x - fixed_value, self.y - fixed_value)
    }

    pub fn mul_scalar(self, value: impl fixed::traits::ToFixed) -> Self {
        let fixed_value = to_fp(value);
        Self::new(self.x * fixed_value, self.y * fixed_value)
    }

    pub fn div_scalar(self, value: impl fixed::traits::ToFixed) -> Self {
        let fixed_value = to_fp(value);
        Self::new(self.x / fixed_value, self.y / fixed_value)
    }

    pub fn abs(&self) -> Self {
        Self::new(abs(self.x), abs(self.y))
    }

    // x^2 + y^2 = length^2
    pub fn length(&self) -> FP {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn dot(self, other: Self) -> FP {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn normalized(&self) -> Self {
        Self::new(self.x / self.length(), self.y / self.length())
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    }
}
