extern crate nalgebra as na;
extern crate simba;

use na::Scalar;
use num::traits::Num;
use simba::scalar::SubsetOf;
use std::ops::Neg;

pub trait Numeric<T>: Num + Copy + Scalar + Neg<Output = T> + PartialOrd {}
impl<T> Numeric<T> for T where T: Num + Copy + Scalar + Neg<Output = T> + PartialOrd {}

pub trait NumericCompConvert<T>: Numeric<T> + SubsetOf<f64> {}
impl<T> NumericCompConvert<T> for T where T: Numeric<T> + SubsetOf<f64> {}
