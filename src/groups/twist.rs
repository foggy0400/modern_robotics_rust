extern crate nalgebra as na;
use crate::interfaces::Numeric;
use na::Vector6;

pub struct Twist<T: Numeric<T>>(pub Vector6<T>);

pub trait ToTwist<T: Numeric<T>> {
    fn to_twist(&self) -> Twist<T>;
}

impl<T: Numeric<T>> ToTwist<T> for Vector6<T> {
    fn to_twist(&self) -> Twist<T> {
        Twist(*self)
    }
}

impl<T: Numeric<T>> ToTwist<T> for [T; 6] {
    fn to_twist(&self) -> Twist<T> {
        Twist(Vector6::from_vec(self.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn to_svvec_array() {
    // let test_mat = Matrix3::new(0.0, -3.3, 2.2, 3.3, 0.0, -1.1, -2.2, 1.1, 0.0);
    // let vec: [f64; 3] = [1.1, 2.2, 3.3];
    // let res = vec.to_so3();
    // assert_eq!(res.0, test_mat);
    // }
}
