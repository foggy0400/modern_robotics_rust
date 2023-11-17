extern crate nalgebra as na;
use crate::bcat;
use crate::concat::{allocate_block_output, Block, HCat, VCat};
use crate::groups::{Se3Matrix, ToSe3, ToSo3};
use crate::interfaces::Numeric;
use na::{Matrix3, Matrix6, Vector6};

pub struct Twist<T: Numeric<T>>(pub Vector6<T>);

pub trait ToTwist<T: Numeric<T>> {
    fn to_twist(&self) -> Twist<T>;
}

pub trait Ad<T: Numeric<T>> {
    fn ad(&self) -> Matrix6<T>;
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

impl<T: Numeric<T>> Ad<T> for Twist<T> {
    fn ad(&self) -> Matrix6<T> {
        let omega = self.0.fixed_view::<3, 1>(0, 0).to_so3();
        let vmat = self.0.fixed_view::<3, 1>(3, 0).to_so3();
        let zeros = Matrix3::zeros();
        return bcat![omega.0, zeros;
                    vmat.0, omega.0];
    }
}

impl<T: Numeric<T>> ToSe3<T> for Twist<T> {
    fn to_se3(&self) -> Se3Matrix<T> {
        self.0.to_se3()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::Matrix4;

    #[test]
    fn array_to_twist() {
        let arr: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let res = arr.to_twist();
        assert_eq!(res.0, Vector6::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6));
    }
    #[test]
    fn vector_to_twist() {
        let vec = Vector6::new(1, 2, 3, 4, 5, 6);
        let res = vec.to_twist();
        assert_eq!(res.0, Vector6::new(1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn se3_from_twist() {
        let test_mat = Matrix4::new(0, -3, 2, 4, 3, 0, -1, 5, -2, 1, 0, 6, 0, 0, 0, 1);
        let vec = Vector6::new(1, 2, 3, 4, 5, 6).to_twist();
        let res = vec.to_se3();
        assert_eq!(res.0, test_mat);
    }
    #[test]
    fn ad() {
        let test_vec = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let res = test_vec.to_twist().ad();
        let test_mat = Matrix6::new(
            0.0, -3.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0, -1.0, 0.0, 0.0, 0.0, -2.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, -6.0, 5.0, 0.0, -3.0, 2.0, 6.0, 0.0, -4.0, 3.0, 0.0, -1.0, -5.0, 4.0, 0.0,
            -2.0, 1.0, 0.0,
        );
        assert_eq!(res, test_mat);
    }
}
