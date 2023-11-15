extern crate nalgebra as na;
use na::Scalar;
use num::traits::Num;
use std::ops::Neg;

pub trait Numeric<T>: Num + Copy + Scalar + Neg<Output = T> {}
impl<T> Numeric<T> for T where T: Num + Copy + Scalar + Neg<Output = T> {}
