extern crate nalgebra as na;
use crate::interfaces::Numeric;
use na::{Matrix4, Vector6};
use num::Zero;

pub struct Se3Matrix<T: Numeric<T>>(pub Matrix4<T>);

pub trait ToSe3<T: Numeric<T>> {
    fn to_se3(&self) -> Se3Matrix<T>;
}

impl<T: Numeric<T>> ToSe3<T> for Matrix4<T> {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(*self)
    }
}

impl<T: Numeric<T>> ToSe3<T> for Vector6<T> {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(Matrix4::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[3],
            self[2],
            Zero::zero(),
            -self[0],
            self[4],
            -self[1],
            self[0],
            Zero::zero(),
            self[5],
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
        ))
    }
}

impl<T: Numeric<T>> ToSe3<T> for [T; 6] {
    fn to_se3(&self) -> Se3Matrix<T> {
        Se3Matrix(Matrix4::new(
            Zero::zero(),
            -self[2],
            self[1],
            self[3],
            self[2],
            Zero::zero(),
            -self[0],
            self[4],
            -self[1],
            self[0],
            Zero::zero(),
            self[5],
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
            Zero::zero(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn se3_array() {
        let test_mat = Matrix4::new(
            0.0, -3.3, 2.2, 4.4, 3.3, 0.0, -1.1, 5.5, -2.2, 1.1, 0.0, 6.6, 0.0, 0.0, 0.0, 0.0,
        );
        let vec: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let res = vec.to_se3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn se3_vector6() {
        let test_mat = Matrix4::new(0, -3, 2, 4, 3, 0, -1, 5, -2, 1, 0, 6, 0, 0, 0, 0);
        let vec = Vector6::new(1, 2, 3, 4, 5, 6);
        let res = vec.to_se3();
        assert_eq!(res.0, test_mat);
    }

    #[test]
    fn se3_matrix() {
        let test_mat = Matrix4::new(
            0.0, -3.0, 2.0, 4.0, 3.0, 0.0, -1.0, 5.0, -2.0, 1.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0,
        );
        let res = test_mat.to_se3();
        assert_eq!(res.0, test_mat);
    }
}
