extern crate nalgebra as na;

use crate::helpers::{near_zero, near_zero_float};
use crate::interfaces::{Numeric, NumericCompConvert};
use na::{ComplexField, Vector3, Vector6};

pub fn axis_ang_3<T: Numeric<T>>(exp: Vector3<T>) -> (Vector3<T>, T)
where
    T: ComplexField<RealField = T>,
{
    let theta = exp.norm();
    (exp / theta, theta)
}

pub fn axis_ang_3_nfloat<T: NumericCompConvert<T>>(exp: Vector3<T>) -> (Vector3<f64>, f64) {
    let exp_c: Vector3<f64> = na::convert(exp);
    let theta = exp_c.norm();
    (exp_c / theta, theta)
}

pub fn axis_ang_6_epsilon<T: Numeric<T> + PartialOrd>(exp: Vector6<T>, eps: T) -> (Vector6<T>, T)
where
    T: ComplexField<RealField = T>,
    f64: From<T>,
{
    let mut theta = exp.fixed_view::<3, 1>(0, 0).norm();
    if near_zero(theta, Some(eps)) {
        theta = exp.fixed_view::<3, 1>(3, 0).norm();
    }
    (exp / theta, theta)
}

pub fn axis_ang_6<T: Numeric<T>>(exp: Vector6<T>) -> (Vector6<T>, T)
where
    T: ComplexField<RealField = T>,
    f64: From<T>,
{
    let mut theta = exp.fixed_view::<3, 1>(0, 0).norm();
    if near_zero(theta, None) {
        theta = exp.fixed_view::<3, 1>(3, 0).norm();
    }
    (exp / theta, theta)
}

pub fn axis_ang_6_epsilon_nfloat<T: NumericCompConvert<T> + PartialOrd>(
    exp: Vector6<T>,
    eps: T,
) -> (Vector6<f64>, f64)
where
    f64: From<T>,
{
    let exp_c: Vector6<f64> = na::convert(exp);
    let mut theta = exp_c.fixed_view::<3, 1>(0, 0).norm();
    if near_zero_float(theta, Some(f64::from(eps))) {
        theta = exp_c.fixed_view::<3, 1>(3, 0).norm();
    }
    (exp_c / theta, theta)
}

pub fn axis_ang_6_nfloat<T: NumericCompConvert<T>>(exp: Vector6<T>) -> (Vector6<f64>, f64)
where
    f64: From<T>,
{
    let exp_c: Vector6<f64> = na::convert(exp);
    let mut theta = exp_c.fixed_view::<3, 1>(0, 0).norm();
    if near_zero_float(theta, None) {
        theta = exp_c.fixed_view::<3, 1>(3, 0).norm();
    }
    (exp_c / theta, theta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_ang_3_f64() {
        let exp = Vector3::new(1.0, 2.0, 3.0);
        let (omg, theta) = axis_ang_3(exp);
        assert!(
            near_zero(
                (omg - Vector3::new(0.2673, 0.5345, 0.8018)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 3.7417, Some(1e-4))
        )
    }

    #[test]
    fn axis_ang_3_i32() {
        let exp = Vector3::new(1, 2, 3);
        let (omg, theta) = axis_ang_3_nfloat(exp);
        assert!(
            near_zero(
                (omg - Vector3::new(0.2673, 0.5345, 0.8018)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 3.7417, Some(1e-4))
        )
    }

    #[test]
    fn axis_ang_6_f64() {
        let exp = Vector6::new(1.0, 0.0, 0.0, 1.0, 2.0, 3.0);
        let (s, theta) = axis_ang_6(exp);
        assert!(
            near_zero(
                (s - Vector6::new(1.0, 0.0, 0.0, 1.0, 2.0, 3.0)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 1.0, Some(1e-4))
        )
    }

    #[test]
    fn axis_ang_6_i32() {
        let exp = Vector6::new(1, 0, 0, 1, 2, 3);
        let (s, theta) = axis_ang_6_nfloat(exp);
        assert!(
            near_zero(
                (s - Vector6::new(1.0, 0.0, 0.0, 1.0, 2.0, 3.0)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 1.0, Some(1e-4))
        )
    }

    #[test]
    fn axis_ang_6_i32_eps() {
        let exp = Vector6::new(1, 0, 0, 2, 0, 0);
        let (s, theta) = axis_ang_6_epsilon_nfloat(exp, 5);
        assert!(
            near_zero(
                (s - Vector6::new(0.5, 0.0, 0.0, 1.0, 0.0, 0.0)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 2.0, Some(1e-4))
        )
    }

    #[test]
    fn axis_ang_6_f64_eps() {
        let exp = Vector6::new(1.0, 0.0, 0.0, 2.0, 0.0, 0.0);
        let (s, theta) = axis_ang_6_epsilon(exp, 5.0);
        assert!(
            near_zero(
                (s - Vector6::new(0.5, 0.0, 0.0, 1.0, 0.0, 0.0)).norm(),
                Some(1e-4)
            ) && near_zero(theta - 2.0, Some(1e-4))
        )
    }
}
